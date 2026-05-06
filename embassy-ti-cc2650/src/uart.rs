#![macro_use]

use core::marker::PhantomData;

use crate::driverlib;
use embassy_hal_internal::Peri;
use embassy_hal_internal::PeripheralType;

const CLOCK_FREQ: u32 = 48_000_000;
pub const BAUD_RATE: u32 = 115_200;

mod internals {
    use core::ops::Deref;

    pub(super) struct Uart(*const cc2650::uart0::RegisterBlock);
    unsafe impl Send for Uart {}
    unsafe impl Sync for Uart {}

    const UART_REGISTER_BLOCK_ADDR: usize = 1073745920;
    pub(super) static UART: Uart = Uart(UART_REGISTER_BLOCK_ADDR as *const _);

    impl Deref for Uart {
        type Target = cc2650::uart0::RegisterBlock;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.0 }
        }
    }
}
use internals::UART;

pub trait UartPinConfig {
    fn tx() -> u32;
    fn rx() -> u32;
    fn rts() -> u32;
    fn cts() -> u32;
}

#[derive(Clone)]
#[non_exhaustive]
pub struct Config {
    pub baudrate: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { baudrate: BAUD_RATE }
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

pub(crate) trait SealedInstance {}

/// UARTE peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    // /// Interrupt for this peripheral.
    //type Interrupt: interrupt::typelevel::Interrupt;
}

macro_rules! impl_uart {
    ($type:ident, $irq:ident) => {
        impl crate::uart::SealedInstance for peripherals::$type {}
        impl crate::uart::Instance for peripherals::$type {
            //       type Interrupt = crate::interrupt::typelevel::$irq;
        }
    };
}

pub struct UartFullRx {}

impl UartFullRx {
    pub fn enable_rx_interrupts(&self) {
        // Set interrupts:
        // - receive interrupt
        // - reception timeout interrupt
        UART.imsc.modify(|_r, w| w.rxim().set_bit().rtim().set_bit())
    }

    pub fn disable_rx_interrupts(&self) {
        // Unset interrupts:
        // - receive interrupt
        // - reception timeout interrupt
        UART.imsc.modify(|_r, w| w.rxim().clear_bit().rtim().clear_bit())
    }

    fn rx_ready(&self) -> bool {
        UART.fr.read().rxff().bit_is_clear()
    }
}

pub struct UartFullTx {}

impl UartFullTx {
    pub fn enable_tx_interrupts(&self) {
        // Set interrupts:
        // - transmit interrupt
        UART.imsc.modify(|_r, w| w.txim().set_bit())
    }

    pub fn disable_tx_interrupts(&self) {
        // Unset interrupts:
        // - transmit interrupt
        UART.imsc.modify(|_r, w| w.txim().clear_bit())
    }

    fn tx_fifo_empty(&self) -> bool {
        UART.fr.read().txfe().bit_is_set()
    }

    fn tx_fifo_full(&self) -> bool {
        UART.fr.read().txff().bit_is_set()
    }
}

pub struct UartFull<'a> {
    rx: UartFullRx,
    tx: UartFullTx,
    _p: PhantomData<&'a ()>,
}

impl<'a> UartFull<'a> {
    // This should only be constructed once
    pub fn new<T: Instance>(uart: Peri<'a, T>, config: Config) -> Self {
        Self::new_inner(uart, config)
    }

    fn new_inner<T: Instance>(_uart: Peri<'a, T>, config: Config) -> Self {
        Self::initialize(config.clone());
        let res = Self {
            rx: UartFullRx {},
            tx: UartFullTx {},
            _p: PhantomData,
        };
        res.set_baud_rate(config.baudrate);
        res.enable();
        res
    }

    /// The idea is that this is only called once per MCU reboot.
    #[inline]
    fn initialize<PinCfg: UartPinConfig>(_pin_cfg: PinCfg) {
        /*
        // 2. Configure the IOC module to map UART signals to the correct GPIO pins.
        // RF1.7_UART_RX EM -> DIO_2
        peripherals
            .IOC
            .iocfg2
            .modify(|_r, w| w.port_id().uart0_rx().ie().set_bit());
        // RF1.9_UART_TX EM -> DIO_3
        peripherals
            .IOC
            .iocfg3
            .modify(|_r, w| w.port_id().uart0_tx().ie().clear_bit());
        */
        unsafe {
            driverlib::IOCPinTypeUart(
                driverlib::UART0_BASE,
                PinCfg::rx(),
                PinCfg::tx(),
                PinCfg::cts(),
                PinCfg::rts(),
            )
        };

        /*
        // For this example, the UART clock is assumed to be 24 MHz, and the desired UART configuration is:
        // • Baud rate: 115 200
        // • Data length of 8 bits
        // • One stop bit
        // • No parity
        // • FIFOs disabled
        // • No interrupts
        //
        // The first thing to consider when programming the UART is the BRD because the UART:IBRD and
        // UART:FBRD registers must be written before the UART:LCRH register.
        // The BRD can be calculated using the equation:
        //      BRD = 24 000 000 / (16 × 115 200) = 13.0208
        // The result of Equation 3 indicates that the UART:IBRD DIVINT field must be set to 13 decimal or 0xD.
        //
        // Equation 4 calculates the value to be loaded into the UART:FBRD register.
        //      UART:FBRD.DIVFRAC = integer (0.0208 × 64 + 0.5) = 1
        //
        // With the BRD values available, the UART configuration is written to the module in the following order:
        let uart = &peripherals.UART0;

        // 1. Disable the UART by clearing the UART:CTL UARTEN bit.
        uart.ctl.modify(|_r, w| w.uarten().dis());

        // 2. Write the integer portion of the BRD to the UART:IBRD register.
        // uart.ibrd.modify(|_r, w| unsafe { w.divint().bits(13) });
        uart.ibrd.modify(|_r, w| unsafe { w.divint().bits(26) }); // for 48 MHz

        // 3. Write the fractional portion of the BRD to the UART:FBRD register.
        // uart.fbrd.modify(|_r, w| unsafe { w.divfrac().bits(1) });
        uart.fbrd.modify(|_r, w| unsafe { w.divfrac().bits(3) }); // for 48 MHz

        // 4. Write the desired serial parameters to the UART:LCRH register (in this case, a value of 0x0000 0060).
        uart.lcrh.modify(|_r, w| w.pen().dis().wlen()._8());

        // 5. Enable the UART by setting the UART:CTL UARTEN bit.
        uart.ctl
            .modify(|_r, w| w.uarten().en().txe().en().rxe().en());
        */

        unsafe {
            driverlib::UARTConfigSetExpClk(
                driverlib::UART0_BASE,
                CLOCK_FREQ,
                BAUD_RATE,
                driverlib::UART_CONFIG_PAR_NONE | driverlib::UART_CONFIG_STOP_ONE | driverlib::UART_CONFIG_WLEN_8,
            )
        };
    }

    fn set_baud_rate(&self, baud_rate: u32) {
        let div = (((CLOCK_FREQ * 8) / baud_rate) + 1) / 2;
        UART.ibrd
            .write(|w| unsafe { w.divint().bits((div / 64).try_into().unwrap()) });
        UART.fbrd
            .write(|w| unsafe { w.divfrac().bits((div % 64).try_into().unwrap()) })
    }

    fn set_hw_flow_control(&self, on: bool) {
        UART.ctl.modify(|_r, w| w.ctsen().bit(on).rtsen().bit(on));
    }

    /// The idea is that this is run each time MCU stops deep sleep.
    fn enable(&self) {
        // UARTEnable is static inline, so better use our own version.
        // unsafe { driverlib::UARTEnable(driverlib::UART0_BASE) }

        // Enable the FIFO.
        UART.lcrh.modify(|_r, w| w.fen().en());

        // Enable RX, TX, and the UART.
        UART.ctl.modify(|_r, w| w.uarten().en().txe().en().rxe().en());
    }

    pub fn split(self) -> (UartTx<'a>, UartRx<'a>) {
        (self.tx, self.rx)
    }

    pub fn split_by_ref(&mut self) -> (&mut UartTx<'a>, &mut UartRx<'a>) {
        (&mut self.tx, &mut self.rx)
    }

    #[allow(unused)]
    pub fn enable_rx_interrupts(&self) {
        self.rx.enable_rx_interrupts();
    }

    #[allow(unused)]
    pub fn enable_tx_interrupts(&self) {
        self.tx.enable_tx_interrupts();
    }

    #[allow(unused)]
    pub fn disable_rx_interrupts(&self) {
        self.rx.disable_rx_interrupts();
    }

    #[allow(unused)]
    pub fn disable_tx_interrupts(&self) {
        self.tx.disable_tx_interrupts();
    }

    /// Transmit one byte at the time
    pub unsafe fn send_byte(&self, byte: u8) {
        UART.dr.write(|w| unsafe { w.data().bits(byte) })
    }

    #[allow(unused)]
    // Pulls a byte out of the RX FIFO.
    #[inline]
    unsafe fn read_byte(&self) -> u8 {
        UART.dr.read().data().bits()
    }

    #[allow(unused)]
    /// Check if the UART transmission is done
    fn tx_fifo_empty(&self) -> bool {
        self.tx.tx_fifo_empty()
    }

    #[allow(unused)]
    /// Check if no more bytes can be enqueued in TX FIFO
    fn tx_fifo_full(&self) -> bool {
        self.tx_fifo_full()
    }

    #[allow(unused)]
    /// Check if either the rx_buffer is full or the UART has timed out
    fn rx_ready(&self) -> bool {
        self.rx.rx_ready()
    }

    pub fn write(&mut self, buffer: &[u8]) {
        for i in 0..buffer.len() {
            while self.tx_fifo_full() {}
            unsafe {
                self.send_byte(buffer[i]);
            }
        }
    }
}

pub(crate) use impl_uart;
