#![macro_use]

use core::marker::PhantomData;
use core::usize;

use crate::chip::interrupt;
use crate::define_peri;
use crate::driverlib;
use crate::driverlib::SysCtrlClockGet;
use crate::driverlib::UARTFIFOEnable;
use crate::driverlib::{UARTDisable, UARTEnable};
use crate::driverlib::{UARTHwFlowControlDisable, UARTHwFlowControlEnable};
use crate::interrupt::typelevel::Interrupt;
use crate::pac;
use crate::udma::UDMA;
use cc2650::uart0::ifls::{RXSELW, TXSELW};
use core::future;
use core::future::poll_fn;
use core::sync::atomic::{Ordering, compiler_fence};
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::waitqueue::AtomicWaker;
use embassy_sync::zerocopy_channel;
use paste::paste;
use static_cell::StaticCell;

// 1073745920 is the start address of registers for UART0.
// cc2650 crate calls it RegisterBlock; I took this
// addres from said crate.
define_peri!(Uart, uart0, 1073745920);

const LF: u8 = b'\n';
const CR: u8 = b'\r';

pub trait UartPinConfig {
    fn tx() -> u32;
    fn rx() -> u32;
    fn rts() -> u32;
    fn cts() -> u32;
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
}

impl Default for Config {
    fn default() -> Self {
        Self {
            hw_flow_control: true,
            baudrate: 115_200,
            fifo_fill_level: FIFOFillLevel::Level48,
        }
    }
}

impl UartPinConfig for Config {
    fn tx() -> u32 {
        driverlib::IOID_3
    }

    fn rx() -> u32 {
        driverlib::IOID_2
    }

    fn rts() -> u32 {
        driverlib::IOID_8
    }

    fn cts() -> u32 {
        driverlib::IOID_4
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
    fn state() -> &'static State;
}

/// UART peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    /// Interrupt for this peripheral.
    type Interrupt: interrupt::typelevel::Interrupt;
}

macro_rules! impl_uart {
    ($type:ident, $irq:ident) => {
        impl crate::uart::SealedInstance for peripherals::$type {
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
pub enum Error {
    /// Buffer was too long.
    BufferTooLong,
    /// Buffer overrun
    Overrun,
    /// Parity error
    Parity,
    /// Framing error
    Framing,
    /// Break condition
    Break,
}

/// Interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let s = T::state();

        // Masked Interrupt Status
        let mis = UART.mis.read();

        // clear interrupt flags
        UART.icr.write(|w| {
            w.beic() // break error
                .set_bit()
                // .ctsmic()            // Clear-To-Send ...
                // .set_bit()
                .feic() // framing error
                .set_bit()
                .oeic() // buffer overrun error
                .set_bit()
                .peic() // parity error
                .set_bit()
                .rtic() // reception timeout
                .set_bit()
                .rxic() // receive
                .set_bit()
                .txic() // transmit
                .set_bit()
        });

        // If an error happened, mask the error interrupts,
        // BUT don't clear the RSR/ECR; it is up to the poller
        // (e.g. reader) to check the error status, clear the status
        // and reenable error interrupts.
        if mis.femis().bit_is_set() // Framing Error
                    || mis.pemis().bit_is_set() // Parity Error
                    || mis.bemis().bit_is_set() // Break Error
                    || mis.oemis().bit_is_set()
        // Overrun Error
        {
            UART.imsc.modify(|_r, w| {
                w.oeim()
                    .clear_bit() // Mask Overrun Error
                    .beim()
                    .clear_bit() // Mask Break Error
                    .peim()
                    .clear_bit() // Mask Parity Error
                    .feim()
                    .clear_bit() // Mask Framing Error
            });
        }

        // UART write complete.
        if UDMA.uart_request_done_tx() {
            UDMA.uart_disable_tx();
            UDMA.uart_request_done_tx_mask();
            UART.dmactl.modify(|_r, w| w.txdmae().clear_bit());
            s.tx_waker.wake();
        }
        // UART rx FIFO limit reached OR rx timeout.
        if mis.rxmis().bit_is_set() || mis.rtmis().bit_is_set() {
            // Mask RX and RT irqs. They should be unmasked by the reader
            // when he ends reading data from the FIFO (e.g. when there is no more data in FIFO).
            unsafe {
                driverlib::UARTIntDisable(driverlib::UART0_BASE, driverlib::UART_INT_RX | driverlib::UART_INT_RT);
            }
            s.rx_waker.wake();
        }
    }
}

fn check_errors() -> Result<(), Error> {
    let rsr = UART.rsr.read();
    if rsr.fe().bit_is_set() || rsr.pe().bit_is_set() || rsr.be().bit_is_set() || rsr.oe().bit_is_set() {
        unsafe { driverlib::UARTRxErrorClear(driverlib::UART0_BASE) };
        if rsr.fe().bit_is_set() {
            Err(Error::Framing)
        } else if rsr.pe().bit_is_set() {
            Err(Error::Parity)
        } else if rsr.be().bit_is_set() {
            Err(Error::Break)
        } else {
            Err(Error::Overrun)
        }
    } else {
        Ok(())
    }
}

/// Trait for the UART receiver. User can implement it for custom behaviour,
/// or use the predefined impl.
pub trait UartFullRxReceiver<'d> {
    /// Initializes a new instance of the receiver. 'rx' is the reading end
    /// of the UART data.
    fn new(rx: zerocopy_channel::Receiver<'d, NoopRawMutex, u8>) -> Self;
    /// Reads data to the 'buf' according to the implemented logic.
    fn read(&mut self, buf: &mut [u8]) -> impl future::Future<Output = usize>;
}

/// Predefined impl of the 'UartFullRxReceiver' trait.
pub struct UartFullRxReceiverImpl<'d> {
    rx: zerocopy_channel::Receiver<'d, NoopRawMutex, u8>,
}

impl<'d> UartFullRxReceiver<'d> for UartFullRxReceiverImpl<'d> {
    fn new(rx: zerocopy_channel::Receiver<'d, NoopRawMutex, u8>) -> Self {
        Self { rx }
    }

    /// Reads bytes into the 'buf' until it encounters 'LF', 'CR' or fills up the whole buffer.
    /// Returns the number of bytes read.
    /// 'LF' and 'CR' are scraped, so this function can return 0.
    fn read(&mut self, buf: &mut [u8]) -> impl future::Future<Output = usize> {
        async {
            let mut ptr = 0usize;
            while ptr != buf.len() {
                let rx_slot = self.rx.receive().await;
                buf[ptr] = *rx_slot;
                rx_slot.receive_done();
                if buf[ptr] == LF || buf[ptr] == CR {
                    return ptr;
                }
                ptr += 1;
            }
            ptr
        }
    }
}

pub struct UartFullRxRunner<'d> {
    state: &'static State,
    tx: zerocopy_channel::Sender<'d, NoopRawMutex, u8>,
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
                *tx_slot = unsafe { driverlib::UARTCharGet(driverlib::UART0_BASE) } as u8;
                tx_slot.send_done();
            }

            // There is no more data, so reenable RX/RT irqs and repeat the loop.
            unsafe {
                driverlib::UARTIntEnable(driverlib::UART0_BASE, driverlib::UART_INT_RX | driverlib::UART_INT_RT);
            }
        }
    }
}

pub struct UartFullTx {
    state: &'static State,
}

impl UartFullTx {
    pub fn configure_tx_interrupts(&self, fill_level: FIFOFillLevel) {
        // Disable UART0 before modifying control registers, as per TI-TRM 19.4.
        UartFull::disable_uart();

        match fill_level {
            FIFOFillLevel::Disabled => {
                // Disable tx interrupt.
                UART.imsc.modify(|_r, w| w.txim().clear_bit());
                UartFull::enable_uart();
                return;
            }
            FIFOFillLevel::Level18 => UART.ifls.modify(|_r, w| w.txsel().variant(TXSELW::_1_8)),
            FIFOFillLevel::Level28 => UART.ifls.modify(|_r, w| w.txsel().variant(TXSELW::_2_8)),
            FIFOFillLevel::Level48 => UART.ifls.modify(|_r, w| w.txsel().variant(TXSELW::_4_8)),
            FIFOFillLevel::Level68 => UART.ifls.modify(|_r, w| w.txsel().variant(TXSELW::_6_8)),
            FIFOFillLevel::Level78 => UART.ifls.modify(|_r, w| w.txsel().variant(TXSELW::_7_8)),
        };

        // Set interrupts:
        // - transmit interrupt
        UART.imsc.modify(|_r, w| w.txim().set_bit());

        UartFull::enable_uart();
    }

    fn tx_fifo_empty(&self) -> bool {
        UART.fr.read().txfe().bit_is_set()
    }

    fn tx_fifo_full(&self) -> bool {
        UART.fr.read().txff().bit_is_set()
    }

    fn dma_start_tx(&self) {
        UART.dmactl.modify(|_r, w| w.txdmae().set_bit());
    }

    #[allow(unused)]
    pub async fn write(&self, buffer: &[u8], tx_len: usize) -> Result<(), Error> {
        if tx_len > driverlib::UDMA_XFER_SIZE_MAX as usize {
            return Err(Error::BufferTooLong);
        }

        UDMA.uart_transfer_tx(&buffer[..tx_len]);

        compiler_fence(Ordering::SeqCst);

        self.dma_start_tx();

        let result = poll_fn(|cx| {
            self.state.tx_waker.register(cx.waker());
            if let Err(e) = check_errors() {
                UDMA.uart_disable_tx();
                return Poll::Ready(Err(e));
            } else if UDMA.uart_request_done_tx() {
                return Poll::Ready(Ok(()));
            }
            Poll::Pending
        })
        .await;

        compiler_fence(Ordering::SeqCst);

        UDMA.uart_request_done_tx_clear();
        UDMA.uart_request_done_tx_unmask();
        result
    }

    /// Same as write(), but instead of async polling it executes busy while() loop.
    #[allow(unused)]
    pub fn write_blocking(&self, buffer: &[u8], tx_len: usize) -> Result<(), Error> {
        if tx_len > driverlib::UDMA_XFER_SIZE_MAX as usize {
            return Err(Error::BufferTooLong);
        }

        UDMA.uart_transfer_tx(&buffer[..tx_len]);

        compiler_fence(Ordering::SeqCst);

        self.dma_start_tx();

        while !UDMA.uart_request_done_tx() {
            if let Err(e) = check_errors() {
                UDMA.uart_disable_tx();
                return Err(e);
            }
        }

        compiler_fence(Ordering::SeqCst);

        UDMA.uart_request_done_tx_clear();
        UDMA.uart_request_done_tx_unmask();
        Ok(())
    }
}

pub struct UartFull<'a> {
    rx_runner: UartFullRxRunner<'a>,
    tx: UartFullTx,
    _p: PhantomData<&'a ()>,
}

impl<'a> UartFull<'a> {
    pub fn new<T: Instance>(
        uart: Peri<'a, T>,
        config: Config,
        _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
    ) -> (Self, zerocopy_channel::Receiver<'static, NoopRawMutex, u8>) {
        Self::new_inner(uart, config)
    }

    fn new_inner<T: Instance>(
        _uart: Peri<'a, T>,
        config: Config,
    ) -> (Self, zerocopy_channel::Receiver<'static, NoopRawMutex, u8>) {
        Self::initialize::<Config>(config);

        static COMM_BUF: StaticCell<[u8; 1024]> = StaticCell::new();
        let comm_buf = COMM_BUF.init([0u8; 1024]);

        static COMM_CH: StaticCell<zerocopy_channel::Channel<'static, NoopRawMutex, u8>> = StaticCell::new();
        let comm_ch = COMM_CH.init(zerocopy_channel::Channel::new(comm_buf));

        let (tx, rx) = comm_ch.split();
        let res = Self {
            rx_runner: UartFullRxRunner { state: T::state(), tx },
            tx: UartFullTx { state: T::state() },
            _p: PhantomData,
        };

        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };
        (res, rx)
    }

    #[inline]
    fn initialize<PinCfg: UartPinConfig>(config: Config) {
        UDMA.enable();

        // Setup IO pins for UART0.
        unsafe {
            driverlib::IOCPinTypeUart(
                driverlib::UART0_BASE,
                PinCfg::rx(),
                PinCfg::tx(),
                PinCfg::cts(),
                PinCfg::rts(),
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
        let mut disabled = false;
        match config.fifo_fill_level {
            FIFOFillLevel::Disabled => disabled = true,
            FIFOFillLevel::Level18 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_1_8)),
            FIFOFillLevel::Level28 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_2_8)),
            FIFOFillLevel::Level48 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_4_8)),
            FIFOFillLevel::Level68 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_6_8)),
            FIFOFillLevel::Level78 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_7_8)),
        };
        if !disabled {
            UART.imsc.modify(|_r, w| w.rxim().set_bit().rtim().set_bit());
        }

        unsafe {
            match config.hw_flow_control {
                true => UARTHwFlowControlEnable(driverlib::UART0_BASE),
                false => UARTHwFlowControlDisable(driverlib::UART0_BASE),
            };
        }

        // Configure channels used for data requests by UART0.
        UDMA.uart_channels_configure();

        // UART uDMA transactions should be only enabled when an actual transmission happens.
        UDMA.uart_disable_tx();

        UartFull::enable_uart();
    }

    fn enable_uart() {
        unsafe {
            UARTEnable(driverlib::UART0_BASE);
        };
    }

    fn disable_uart() {
        unsafe {
            UARTDisable(driverlib::UART0_BASE);
        };
    }

    #[allow(unused)]
    pub fn split(self) -> (UartFullTx, UartFullRxRunner<'a>) {
        (self.tx, self.rx_runner)
    }

    #[allow(unused)]
    pub fn configure_rx_interrupts(&self, fill_level: FIFOFillLevel) {
        // Disable UART0 before modifying control registers, as per TI-TRM 19.4.
        UartFull::disable_uart();

        match fill_level {
            FIFOFillLevel::Disabled => {
                // Disable interrupts:
                // - receive interrupt
                // - reception timeout interrupt
                UART.imsc.modify(|_r, w| w.rxim().clear_bit().rtim().clear_bit());
                UartFull::enable_uart();
                return;
            }
            FIFOFillLevel::Level18 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_1_8)),
            FIFOFillLevel::Level28 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_2_8)),
            FIFOFillLevel::Level48 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_4_8)),
            FIFOFillLevel::Level68 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_6_8)),
            FIFOFillLevel::Level78 => UART.ifls.modify(|_r, w| w.rxsel().variant(RXSELW::_7_8)),
        };
        // Set interrupts:
        // - receive interrupt
        // - reception timeout interrupt
        UART.imsc.modify(|_r, w| w.rxim().set_bit().rtim().set_bit());

        UartFull::enable_uart();
    }

    #[allow(unused)]
    pub fn configure_tx_interrupts(&self, fill_level: FIFOFillLevel) {
        self.tx.configure_tx_interrupts(fill_level);
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
        UART.fr.read().rxfe().bit_is_set()
    }

    #[allow(unused)]
    /// Check if RX FIFO is full
    fn rx_fifo_full(&self) -> bool {
        UART.fr.read().rxff().bit_is_set()
    }

    #[allow(unused)]
    pub async fn write(&self, buffer: &[u8], tx_len: usize) -> Result<(), Error> {
        self.tx.write(buffer, tx_len).await
    }

    /// Same as write(), but instead of async polling it executes busy while() loop.
    #[allow(unused)]
    pub fn write_blocking(&self, buffer: &[u8], tx_len: usize) -> Result<(), Error> {
        self.tx.write_blocking(buffer, tx_len)
    }
}

pub(crate) use impl_uart;
