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
pub mod rtc;
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

macro_rules! define_peri {
    ($name:ident, $cc2650_crate:ident, $addr:expr) => {
        mod internals {
            use super::pac;
            use super::paste;
            use core::ops::Deref;

            #[allow(non_camel_case_types)]
            pub(super) struct $name(*const pac::$cc2650_crate::RegisterBlock);
            unsafe impl Send for $name {}
            unsafe impl Sync for $name {}

            paste! {
const [<$name:upper _REGISTER_BLOCK_ADDR>]: usize = $addr;
                pub(super) static [<$name:upper>]: $name = $name([<$name:upper _REGISTER_BLOCK_ADDR>] as *const _);
            }

            impl Deref for $name {
                type Target = pac::$cc2650_crate::RegisterBlock;

                // SAFETY: self.0 is an address of the start of the specific peripheral's registers.
                // It should be taken from the cc2650 crate directly, as this crate wraps
                // this address into RegisterBlock. As a result, as long as the addres is taken
                // from the cc2650 crate and binded to the correct peripheral from this crate,
                // this deref impl should be "safe".
                fn deref(&self) -> &Self::Target {
                    unsafe { &*self.0 }
                }
            }
        }
        paste! { use internals::[<$name:upper>]; }
    };
}
pub(crate) use define_peri;

pub trait PinConfig: UartPinConfig + Copy {}
impl<T> PinConfig for T where T: UartPinConfig + Copy {}

pub fn init() -> Peripherals {
    let peripherals = pac::Peripherals::take().unwrap();
    let prcm = Prcm::new(peripherals.PRCM);

    prcm.rfc_modesel_configure();

    prcm.enable_domains(prcm::PowerDomains::empty().peripherals().serial());

    prcm.enable_clocks(prcm::Clocks::empty().gpio().uart().gpt().dma().crypto().i2c());

    Peripherals::take()
}
