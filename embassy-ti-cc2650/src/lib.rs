#![no_std]
#![crate_name = "embassy_ti_cc2650"]
#![crate_type = "rlib"]
#![warn(unreachable_pub)]

use crate::chip::Peripherals;
use crate::prcm::Prcm;
use crate::uart::UartPinConfig;

mod ccfg;
pub mod chip;
pub mod driverlib;
pub mod gpio;
pub mod gpt;
pub mod prcm;
#[cfg(not(feature = "embassy-time"))]
pub mod rtc;
#[cfg(feature = "embassy-time")]
pub mod time_driver;
pub mod uart;
pub mod udma;

pub use crate::chip::interrupt;
pub(crate) use chip::pac;

// developer note: this macro can't be in `embassy-hal-internal` due to the use of `$crate`.
#[macro_export]
macro_rules! bind_interrupts {
    ($(#[$attr:meta])* $vis:vis struct $name:ident {
        $(
            $(#[cfg($cond_irq:meta)])?
            $irq:ident => $(
                $(#[cfg($cond_handler:meta)])?
                $handler:ty
            ),*;
        )*
    }) => {
        #[derive(Copy, Clone)]
        $(#[$attr])*
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            $(#[cfg($cond_irq)])?
            unsafe extern "C" fn $irq() {
                unsafe {
                    $(
                        $(#[cfg($cond_handler)])?
                        <$handler as $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$irq>>::on_interrupt();

                    )*
                }
            }
            $(#[cfg($cond_irq)])?
            $crate::bind_interrupts!(@inner
                $(
                    $(#[cfg($cond_handler)])?
                    unsafe impl $crate::interrupt::typelevel::Binding<$crate::interrupt::typelevel::$irq, $handler> for $name {}
                )*
            );
        )*
    };
    (@inner $($t:tt)*) => {
        $($t)*
    }
}

pub trait PinConfig: UartPinConfig + Copy {}
impl<T> PinConfig for T where T: UartPinConfig + Copy {}

pub fn init() -> Peripherals {
    let peripherals = Peripherals::take();
    let prcm = Prcm::new();

    prcm.rfc_modesel_configure();

    prcm.enable_domains(prcm::PowerDomains::empty().peripherals().serial());

    prcm.enable_clocks(prcm::Clocks::empty().gpio().uart().gpt().dma().crypto().i2c());

    #[cfg(feature = "embassy-time")]
    time_driver::init();

    peripherals
}
