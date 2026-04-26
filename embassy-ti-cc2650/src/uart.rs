/// Stores an ongoing TX/RX transaction
struct Transaction {
    /// The buffer containing the bytes to transmit as it should be returned to
    /// the client
    buffer: &'static mut [u8],
    /// The total amount to transmit
    length: usize,
    /// The index of the byte currently being sent
    index: usize,
}

use core::cell::Cell;

use kernel::{ErrorCode, hil};
use tock_cells::{map_cell::MapCell, optional_cell::OptionalCell};

use crate::{driverlib, udma};

use super::Transaction;

// 48 MHz
const CLOCK_FREQ: u32 = 48_000_000;
pub const BAUD_RATE: u32 = 115_200;

pub trait UartPinConfig {
    fn tx() -> u32;
    fn rx() -> u32;
    fn rts() -> u32;
    fn cts() -> u32;
}

#[derive(Copy, Clone)]
pub enum Parity {
    None = 0,
    Odd = 1,
    Even = 2,
}

#[derive(Clone, Copy)]
pub struct Config {
    pub baud_rate: u32,
    pub clock_freq: u32,
    pub parity: Parity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            baud_rate: BAUD_RATE,
            clock_freq: CLOCK_FREQ,
            parity: Parity::None,
        }
    }
}

impl UartPinConfig for Config {
    fn tx() -> u32 {
        cc2650_chip::driverlib::IOID_3
    }

    fn rx() -> u32 {
        cc2650_chip::driverlib::IOID_2
    }

    fn rts() -> u32 {
        cc2650_chip::driverlib::IOID_8
    }

    fn cts() -> u32 {
        cc2650_chip::driverlib::IOID_4
    }
}

pub(crate) trait SealedInstance {
    fn regs() -> cc2650::UART0;
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    type Interrupt: interrupt::typelevel::Interrupt;
}

pub struct UartFull<'a> {
    r: cc2650::UART0,
}

impl<'a> UartFull<'a> {
    pub fn new(uart: Peri<'a, cc2650::UART0>, udma: Peri<'a, udma::Udma>, config: Config) -> Self {
        new_inner(uart, udma, config)
    }

    fn new_inner<T: Instance>(uart: Peri<'a, T>, udma: Peri<'a, T>, config: Config) -> Self {
        self.configure(config);
        self.initialize(config);
        self.enable();
        Self { r: T::regs() }
    }

    #[inline]
    pub fn initialize<PinCfg: UartPinConfig>(&self, _pin_cfg: PinCfg) {
        unsafe {
            driverlib::IOCPinTypeUart(
                driverlib::UART0_BASE,
                PinCfg::rx(),
                PinCfg::tx(),
                PinCfg::cts(),
                PinCfg::rts(),
            )
        };

        unsafe {
            driverlib::UARTConfigSetExpClk(
                driverlib::UART0_BASE,
                CLOCK_FREQ,
                BAUD_RATE,
                driverlib::UART_CONFIG_PAR_NONE | driverlib::UART_CONFIG_STOP_ONE | driverlib::UART_CONFIG_WLEN_8,
            )
        };

        self.udma.uart_channels_configure();
    }

    fn set_baud_rate(&self, baud_rate: u32) {
        let div = (((CLOCK_FREQ * 8) / baud_rate) + 1) / 2;
        self.uart
            .ibrd
            .write(|w| unsafe { w.divint().bits((div / 64).try_into().unwrap()) });
        self.uart
            .fbrd
            .write(|w| unsafe { w.divfrac().bits((div % 64).try_into().unwrap()) })
    }

    fn configure(&self, params: &Config) -> Result<(), ErrorCode> {
        // These could probably be implemented, but are currently ignored,
        // so throw an error.

        if params.parity != hil::uart::Parity::None {
            return Err(ErrorCode::NOSUPPORT);
        }

        if params.baud_rate == 0 {
            return Err(ErrorCode::INVAL);
        }
        self.set_baud_rate(params.baud_rate);

        Ok(())
    }

    /// The idea is that this is run each time MCU stops deep sleep.
    pub fn enable(&self) {
        // Disable, because they should be enabled only upon a transfer/receive request.
        self.udma.uart_disable_tx();
        self.udma.uart_disable_rx();

        // UARTEnable is static inline, so better use our own version.
        // unsafe { driverlib::UARTEnable(driverlib::UART0_BASE) }

        // Enable the FIFO.
        self.uart.lcrh.modify(|_r, w| w.fen().en());

        // Enable RX, TX, and the UART.
        self.uart.ctl.modify(|_r, w| w.uarten().en().txe().en().rxe().en());
    }

    #[allow(dead_code)]
    pub fn disable(&self) {
        self.dma_stop_tx();
        self.udma.uart_disable_tx();
        self.udma.uart_disable_rx();
        unsafe { driverlib::UARTDisable(driverlib::UART0_BASE) };
    }

    fn dma_start_tx(&self) {
        self.uart.dmactl.modify(|_r, w| w.txdmae().set_bit());
    }

    fn dma_start_rx(&self) {
        self.uart.dmactl.modify(|_r, w| w.rxdmae().set_bit());
    }

    fn dma_stop_tx(&self) {
        self.udma.uart_disable_tx();
        self.uart.dmactl.modify(|_r, w| w.txdmae().clear_bit());
    }

    fn dma_stop_rx(&self) {
        self.udma.uart_disable_rx();
        self.uart.dmactl.modify(|_r, w| w.rxdmae().clear_bit());
    }

    #[allow(unused)]
    fn enable_rx_interrupts(&self) {
        // Set interrupts:
        // - receive interrupt
        // - reception timeout interrupt
        self.uart.imsc.modify(|_r, w| w.rxim().set_bit().rtim().set_bit())
    }

    #[allow(unused)]
    fn enable_tx_interrupts(&self) {
        // Set interrupts:
        // - transmit interrupt
        self.uart.imsc.modify(|_r, w| w.txim().set_bit())
    }

    #[allow(unused)]
    fn disable_rx_interrupts(&self) {
        // Unset interrupts:
        // - receive interrupt
        // - reception timeout interrupt
        self.uart.imsc.modify(|_r, w| w.rxim().clear_bit().rtim().clear_bit())
    }

    #[allow(unused)]
    fn disable_tx_interrupts(&self) {
        // Unset interrupts:
        // - transmit interrupt
        self.uart.imsc.modify(|_r, w| w.txim().clear_bit())
    }
}
