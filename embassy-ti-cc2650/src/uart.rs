#![macro_use]

use crate::chip::interrupt;
use crate::driverlib;
use crate::driverlib::SysCtrlClockGet;
use crate::driverlib::UARTFIFOEnable;
use crate::driverlib::{UARTDisable, UARTEnable};
use crate::driverlib::{UARTHwFlowControlDisable, UARTHwFlowControlEnable};
use crate::interrupt::typelevel::Interrupt;
use crate::pac;
use crate::udma::UDMA;
use core::future;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::ptr::addr_of;
use core::sync::atomic::{Ordering, compiler_fence};
use core::task::Poll;
use core::usize;
use embassy_hal_internal::drop::OnDrop;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_sync::zerocopy_channel;
use static_cell::StaticCell;

const LF: u8 = b'\n';
const CR: u8 = b'\r';

const FE: u8 = 0b1;
const PE: u8 = 0b10;
const BE: u8 = 0b100;
const OE: u8 = 0b1000;

pub trait UartPinConfig {
    fn tx() -> u32;
    fn rx() -> u32;
    fn rts(hw: bool) -> u32;
    fn cts(hw: bool) -> u32;
}

#[derive(Clone)]
pub enum FIFOFillLevel {
    /// Transmit/Receive FIFO disabled,
    Disabled,
    /// Transmit/Receive FIFO becomes >= 1/8 full
    Level18,
    /// Transmit/Receive FIFO becomes >= 2/8 full
    Level28,
    /// Transmit/Receive FIFO becomes >= 4/8 full
    Level48,
    /// Transmit/Receive FIFO becomes >= 6/8 full
    Level68,
    /// Transmit/Receive FIFO becomes >= 7/8 full
    Level78,
}

#[derive(Clone)]
#[non_exhaustive]
pub struct Config {
    pub hw_flow_control: bool,
    pub baudrate: u32,
    pub fifo_fill_level: FIFOFillLevel,
    pub tx_pin: u32,
    pub rx_pin: u32,
    pub rts_pin: u32,
    pub cts_pin: u32,
}

impl Config {
    fn rts(&self) -> u32 {
        match self.hw_flow_control {
            true => self.rts_pin,
            false => driverlib::IOID_UNUSED,
        }
    }

    fn cts(&self) -> u32 {
        match self.hw_flow_control {
            true => self.cts_pin,
            false => driverlib::IOID_UNUSED,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hw_flow_control: true,
            baudrate: 115_200,
            fifo_fill_level: FIFOFillLevel::Level48,
            tx_pin: driverlib::IOID_3,
            rx_pin: driverlib::IOID_2,
            rts_pin: driverlib::IOID_8,
            cts_pin: driverlib::IOID_4,
        }
    }
}

pub(crate) struct State {
    pub(crate) rx_waker: AtomicWaker,
    pub(crate) tx_waker: AtomicWaker,
}

impl State {
    pub(crate) const fn new() -> Self {
        Self {
            rx_waker: AtomicWaker::new(),
            tx_waker: AtomicWaker::new(),
        }
    }
}

pub(crate) trait SealedInstance {
    fn regs() -> pac::UART0::UART0;
    fn state() -> &'static State;
}

/// UART peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    /// Interrupt for this peripheral.
    type Interrupt: interrupt::typelevel::Interrupt;
}

macro_rules! impl_uart {
    ($type:ident, $pac_type:ident, $irq:ident) => {
        impl crate::uart::SealedInstance for peripherals::$type {
            fn regs() -> pac::UART0::UART0 {
                pac::$pac_type
            }
            fn state() -> &'static crate::uart::State {
                static STATE: crate::uart::State = crate::uart::State::new();
                &STATE
            }
        }
        impl crate::uart::Instance for peripherals::$type {
            type Interrupt = crate::interrupt::typelevel::$irq;
        }
    };
}

#[derive(Debug)]
pub enum RxError {
    /// Buffer overrun
    Overrun(usize),
    /// Parity error
    Parity(usize),
    /// Framing error
    Framing(usize),
    /// Break condition
    Break(usize),
}

#[derive(Debug)]
pub enum TxError {
    ///Buffer was empty
    BufferEmpty,
    /// Buffer was too long.
    BufferTooLong,
    /// Buffer was coming from FLASH memory
    SramMemory,
}

fn enable_uart_irqs(flags: u32) {
    unsafe {
        driverlib::UARTIntEnable(driverlib::UART0_BASE, flags);
    }
}

fn disable_uart_irqs(flags: u32) {
    unsafe {
        driverlib::UARTIntDisable(driverlib::UART0_BASE, flags);
    }
}

/// Interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let r = T::regs();
        let s = T::state();

        // Masked Interrupt Status
        let mis = r.MIS().read();

        // clear interrupt flags
        r.ICR().write(|w| {
            w.set_RTIC(true); // receive timeout
            w.set_RXIC(true); // receive
        });

        // UART write complete.
        if UDMA.uart_request_done_tx() {
            UDMA.uart_disable_tx();
            UDMA.uart_request_done_tx_mask();
            r.DMACTL().modify(|w| w.set_TXDMAE(false));
            s.tx_waker.wake();
        }
        // UART rx FIFO limit reached OR rx timeout.
        if mis.RXMIS() || mis.RTMIS() {
            // Mask RX and RT irqs. They should be unmasked by the reader
            // when he ends reading data from the FIFO (e.g. when there is no more data in FIFO).
            disable_uart_irqs(driverlib::UART_INT_RX | driverlib::UART_INT_RT);
            s.rx_waker.wake();
        }
    }
}

/// Trait for the UART receiver. User can implement it for custom behaviour,
/// or use the predefined impl.
pub trait UartFullRxReceiver<'d> {
    /// Initializes a new instance of the receiver. 'rx' is the reading end
    /// of the UART data.
    fn new(rx: zerocopy_channel::Receiver<'d, NoopRawMutex, u16>) -> Self;
    /// Reads data to the 'buf' according to the implemented logic.
    fn read(&mut self, buf: &mut [u8]) -> impl future::Future<Output = Result<usize, RxError>>;
}

/// Predefined impl of the 'UartFullRxReceiver' trait.
pub struct UartFullRxReceiverImpl<'d> {
    rx: zerocopy_channel::Receiver<'d, NoopRawMutex, u16>,
}

impl<'d> UartFullRxReceiverImpl<'d> {
    fn check_errors(rsr: u8, ptr: usize) -> Result<usize, RxError> {
        if rsr & FE != 0 {
            Err(RxError::Framing(ptr))
        } else if rsr & PE != 0 {
            Err(RxError::Parity(ptr))
        } else if rsr & BE != 0 {
            Err(RxError::Break(ptr))
        } else if rsr & OE != 0 {
            Err(RxError::Overrun(ptr))
        } else {
            Ok(ptr)
        }
    }
}

impl<'d> UartFullRxReceiver<'d> for UartFullRxReceiverImpl<'d> {
    fn new(rx: zerocopy_channel::Receiver<'d, NoopRawMutex, u16>) -> Self {
        Self { rx }
    }

    /// Reads bytes into the 'buf' until it encounters 'LF', 'CR' or fills up the whole buffer.
    /// Returns the number of bytes read, and possibly RX error.
    /// 'LF' and 'CR' are scraped, so this function can return 0.
    fn read(&mut self, buf: &mut [u8]) -> impl future::Future<Output = Result<usize, RxError>> {
        async {
            let mut ptr = 0usize;
            while ptr != buf.len() {
                let rx_slot = self.rx.receive().await;
                let rx_slot_bytes = (*rx_slot).to_le_bytes();
                buf[ptr] = rx_slot_bytes[0];
                rx_slot.receive_done();
                if buf[ptr] == LF || buf[ptr] == CR {
                    return Ok(ptr);
                }
                ptr += 1;
                let res = Self::check_errors(rx_slot_bytes[1], ptr);
                if res.is_err() {
                    return res;
                }
            }
            Ok(ptr)
        }
    }
}

pub struct UartFullRxRunner<'d> {
    state: &'static State,
    tx: zerocopy_channel::Sender<'d, NoopRawMutex, u16>,
}

impl<'d> UartFullRxRunner<'d> {
    pub async fn run(&mut self) {
        loop {
            // Wait for data to "show up".
            let _ = poll_fn(|cx| {
                self.state.rx_waker.register(cx.waker());
                if unsafe { driverlib::UARTCharsAvail(driverlib::UART0_BASE) } {
                    return Poll::Ready(());
                }
                Poll::Pending
            })
            .await;

            // While there is data in the FIFO, consume it and push to the reader.
            // This also covers data that arrived AFTER RX/RT interrupt fired.
            while unsafe { driverlib::UARTCharsAvail(driverlib::UART0_BASE) } {
                let mut tx_slot = self.tx.send().await;
                // 8 bits for data and 4 bits for errors -> u16.
                *tx_slot = unsafe { driverlib::UARTCharGet(driverlib::UART0_BASE) } as u16;
                tx_slot.send_done();
                unsafe { driverlib::UARTRxErrorClear(driverlib::UART0_BASE) };
            }

            // There is no more data, so reenable RX/RT irqs and repeat the loop.
            enable_uart_irqs(driverlib::UART_INT_RX | driverlib::UART_INT_RT);
        }
    }
}

pub struct UartFullTx<T: Instance> {
    state: &'static State,
    _p: PhantomData<T>,
}

impl<T: Instance> UartFullTx<T> {
    fn tx_fifo_empty(&self) -> bool {
        T::regs().FR().read().TXFE()
    }

    fn tx_fifo_full(&self) -> bool {
        T::regs().FR().read().TXFF()
    }

    fn dma_start_tx(&self) {
        T::regs().DMACTL().modify(|w| w.set_TXDMAE(true));
    }

    fn sanitize_tx_input_buffer(&self, buffer: &[u8]) -> Result<(), TxError> {
        if buffer.len() > driverlib::UDMA_XFER_SIZE_MAX as usize {
            return Err(TxError::BufferTooLong);
        }
        if buffer.len() == 0 {
            return Err(TxError::BufferEmpty);
        }
        if (addr_of!(buffer) as u32) < driverlib::SRAM_BASE {
            return Err(TxError::SramMemory);
        }
        Ok(())
    }

    /// uDMA-based, asynchronous write. Each write should be smaller than UDMA_XFER_SIZE_MAX ,
    /// and because of uDMA limitations, data should not come from the FLASH memory.
    #[allow(unused)]
    pub async fn write(&mut self, buffer: &[u8]) -> Result<(), TxError> {
        let res = self.sanitize_tx_input_buffer(buffer);
        if res.is_err() {
            return res;
        }

        let drop = OnDrop::new(move || {
            UDMA.uart_disable_tx();
            T::regs().DMACTL().modify(|w| w.set_TXDMAE(false));
            UDMA.uart_request_done_tx_clear();
            UDMA.uart_request_done_tx_unmask();
        });

        UDMA.uart_transfer_tx(buffer);

        compiler_fence(Ordering::SeqCst);

        self.dma_start_tx();

        let result = poll_fn(|cx| {
            self.state.tx_waker.register(cx.waker());
            if UDMA.uart_request_done_tx() {
                return Poll::Ready(Ok(()));
            }
            Poll::Pending
        })
        .await;

        compiler_fence(Ordering::SeqCst);

        UDMA.uart_request_done_tx_clear();
        UDMA.uart_request_done_tx_unmask();
        drop.defuse();
        result
    }

    /// Same as write(), but instead of async polling it executes busy while() loop.
    #[allow(unused)]
    pub fn write_blocking(&mut self, buffer: &[u8]) -> Result<(), TxError> {
        let res = self.sanitize_tx_input_buffer(buffer);
        if res.is_err() {
            return res;
        }
        UDMA.uart_transfer_tx(buffer);

        compiler_fence(Ordering::SeqCst);

        self.dma_start_tx();

        while !UDMA.uart_request_done_tx() {}

        compiler_fence(Ordering::SeqCst);

        UDMA.uart_request_done_tx_clear();
        UDMA.uart_request_done_tx_unmask();
        Ok(())
    }
}

pub struct UartFull<'a, T: Instance> {
    rx_runner: UartFullRxRunner<'a>,
    tx: UartFullTx<T>,
    _peri: Peri<'a, T>,
}

impl<'a, T: Instance> UartFull<'a, T> {
    pub fn new(
        uart: Peri<'a, T>,
        config: Config,
        comm_buf: &'static mut [u16],
        _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
    ) -> (Self, zerocopy_channel::Receiver<'static, NoopRawMutex, u16>) {
        Self::initialize(config);

        static COMM_CH: StaticCell<zerocopy_channel::Channel<'static, NoopRawMutex, u16>> = StaticCell::new();
        let comm_ch = COMM_CH.init(zerocopy_channel::Channel::new(comm_buf));

        let (tx, rx) = comm_ch.split();
        let res = Self {
            rx_runner: UartFullRxRunner { state: T::state(), tx },
            tx: UartFullTx::<T> {
                state: T::state(),
                _p: PhantomData,
            },
            _peri: uart,
        };

        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };
        (res, rx)
    }

    #[inline]
    fn initialize(config: Config) {
        let r = T::regs();
        UDMA.enable();

        // Setup IO pins for UART0.
        unsafe {
            driverlib::IOCPinTypeUart(
                driverlib::UART0_BASE,
                config.rx_pin,
                config.tx_pin,
                config.cts(),
                config.rts(),
            )
        };

        // Setup UART. This also disables UART0, as required by TI-TRM 19.4.
        unsafe {
            driverlib::UARTConfigSetExpClk(
                driverlib::UART0_BASE,
                SysCtrlClockGet(),
                config.baudrate,
                driverlib::UART_CONFIG_PAR_NONE | driverlib::UART_CONFIG_STOP_ONE | driverlib::UART_CONFIG_WLEN_8,
            )
        };

        // Enable FIFO.
        unsafe {
            UARTFIFOEnable(driverlib::UART0_BASE);
        };

        // Configure RX interrupts. TX interrupts are disabled by default as TX is handled by uDMA.
        disable_uart_irqs(driverlib::UART_INT_TX);
        let mut disabled = false;
        match config.fifo_fill_level {
            FIFOFillLevel::Disabled => disabled = true,
            FIFOFillLevel::Level18 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_1_8)),
            FIFOFillLevel::Level28 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_2_8)),
            FIFOFillLevel::Level48 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_4_8)),
            FIFOFillLevel::Level68 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_6_8)),
            FIFOFillLevel::Level78 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_7_8)),
        };
        if !disabled {
            enable_uart_irqs(driverlib::UART_INT_RX | driverlib::UART_INT_RT);
        }

        // Disable clear-to-send irq as uDMA handles TX.
        disable_uart_irqs(driverlib::UART_INT_CTS);

        // Disable error irqs. Errors will be returned on read from the FIFO.
        disable_uart_irqs(
            driverlib::UART_INT_OE | driverlib::UART_INT_BE | driverlib::UART_INT_PE | driverlib::UART_INT_FE,
        );

        unsafe {
            match config.hw_flow_control {
                true => UARTHwFlowControlEnable(driverlib::UART0_BASE),
                false => UARTHwFlowControlDisable(driverlib::UART0_BASE),
            };
        }

        // Configure channels used for data requests by UART0.
        UDMA.uart_tx_channel_configure();

        // UART uDMA transactions should be only enabled when an actual transmission happens.
        UDMA.uart_disable_tx();

        UartFull::<T>::enable_uart();
    }

    #[allow(unused)]
    pub fn split(self) -> (UartFullTx<T>, UartFullRxRunner<'a>) {
        (self.tx, self.rx_runner)
    }

    #[allow(unused)]
    /// This function will wait until there is no more data to send,
    /// but it won't wait for the RX FIFO (data might be lost).
    pub fn configure_rx_interrupts(&self, fill_level: FIFOFillLevel) {
        let r = T::regs();
        // Disable UART0 before modifying control registers, as per TI-TRM 19.4.
        UartFull::<T>::disable_uart();

        match fill_level {
            FIFOFillLevel::Disabled => {
                // Disable interrupts:
                // - receive interrupt
                // - reception timeout interrupt
                disable_uart_irqs(driverlib::UART_INT_RX | driverlib::UART_INT_RT);
                r.CTL().modify(|w| w.set_UARTEN(vals::UARTEN::EN));
                return;
            }
            FIFOFillLevel::Level18 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_1_8)),
            FIFOFillLevel::Level28 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_2_8)),
            FIFOFillLevel::Level48 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_4_8)),
            FIFOFillLevel::Level68 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_6_8)),
            FIFOFillLevel::Level78 => r.IFLS().modify(|w| w.set_RXSEL(vals::RXSEL::_7_8)),
        };
        // Set interrupts:
        // - receive interrupt
        // - reception timeout interrupt
        enable_uart_irqs(driverlib::UART_INT_RX | driverlib::UART_INT_RT);

        UartFull::<T>::enable_uart();
    }

    fn enable_uart() {
        unsafe {
            UARTEnable(driverlib::UART0_BASE);
        };
    }

    /// UARTDisable waits until the BUSY flag for TX is cleared.
    /// According to TI-TRM 19.4.3, this happens only, when:
    /// 1. TX FIFO empty &&
    /// 2. Last character was transmitted from the shift register.
    /// So, this function waits for TX FIFO to be empty.
    fn disable_uart() {
        unsafe {
            UARTDisable(driverlib::UART0_BASE);
        };
    }

    #[allow(unused)]
    /// Check if the TX FIFO is empty
    fn tx_fifo_empty(&self) -> bool {
        self.tx.tx_fifo_empty()
    }

    #[allow(unused)]
    /// Check if no more bytes can be enqueued in TX FIFO
    fn tx_fifo_full(&self) -> bool {
        self.tx.tx_fifo_full()
    }

    #[allow(unused)]
    /// Check if RX FIFO is empty
    fn rx_fifo_empty(&self) -> bool {
        T::regs().FR().read().RXFE()
    }

    #[allow(unused)]
    /// Check if RX FIFO is full
    fn rx_fifo_full(&self) -> bool {
        T::regs().FR().read().RXFF()
    }

    #[allow(unused)]
    pub async fn write(&mut self, buffer: &[u8]) -> Result<(), TxError> {
        self.tx.write(buffer).await
    }

    /// Same as write(), but instead of async polling it executes busy while() loop.
    #[allow(unused)]
    pub fn write_blocking(&mut self, buffer: &[u8]) -> Result<(), TxError> {
        self.tx.write_blocking(buffer)
    }
}

pub(crate) use impl_uart;
use ti_cc2650_pac::UART0::vals;
