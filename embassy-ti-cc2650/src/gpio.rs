#![macro_use]

use crate::define_peri;
use crate::driverlib;
use crate::pac;
use embassy_hal_internal::Peri;
use embassy_hal_internal::PeripheralType;
use embassy_hal_internal::impl_peripheral;
use paste::paste;

// 1073881088 is the start address of registers for GPIO.
// cc2650 crate calls it RegisterBlock; I took this
// addres from said crate.
define_peri!(Gpio, gpio, 1073881088);

/// Pull setting for an input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pull {
    /// No pull.
    None,
    /// Internal pull-up resistor.
    Up,
    /// Internal pull-down resistor.
    Down,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Level {
    /// Logical low.
    Low,
    /// Logical high.
    High,
}

pub struct Input<'d> {
    pub(crate) gpio_pin: GPIOPin<'d>,
}

impl<'d> Input<'d> {
    /// Create GPIO input driver for a [Pin] with the provided [Pull] configuration.
    #[inline]
    pub fn new(pin: Peri<'d, impl Pin>, pull: Pull) -> Self {
        let mut gpio_pin = GPIOPin::new(pin);
        gpio_pin.make_input(pull);
        Self { gpio_pin }
    }

    /// Get whether the pin input level is high.
    #[inline]
    pub fn is_high(&self) -> bool {
        self.gpio_pin.is_high()
    }

    /// Get whether the pin input level is low.
    #[inline]
    pub fn is_low(&self) -> bool {
        self.gpio_pin.is_low()
    }
}

pub struct Output<'d> {
    pub(crate) gpio_pin: GPIOPin<'d>,
}

impl<'d> Output<'d> {
    pub fn new(pin: Peri<'d, impl Pin>, initial_output: Level) -> Self {
        let mut gpio_pin = GPIOPin::new(pin);
        gpio_pin.make_output();
        match initial_output {
            Level::Low => gpio_pin.set_low(),
            Level::High => gpio_pin.set_high(),
        };
        Self { gpio_pin }
    }

    /// Set the output as high.
    #[inline]
    pub fn set_high(&mut self) {
        self.gpio_pin.set_high()
    }

    /// Set the output as low.
    #[inline]
    pub fn set_low(&mut self) {
        self.gpio_pin.set_low()
    }

    /// Get whether the output level is set to high.
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.gpio_pin.is_set_high()
    }

    /// Get whether the output level is set to low.
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.gpio_pin.is_set_low()
    }
}

pub(crate) trait SealedPin {
    fn pin_port(&self) -> u32;
}

#[allow(private_bounds)]
pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    #[inline]
    fn pin(&self) -> u32 {
        self.pin_port()
    }
}

pub(crate) struct GPIOPin<'d> {
    pin: Peri<'d, AnyPin>,
}

impl<'d> GPIOPin<'d> {
    fn new(pin: Peri<'d, impl Pin>) -> Self {
        Self { pin: pin.into() }
    }

    fn pin_mask(&self) -> u32 {
        1 << self.pin.pin()
    }

    fn make_input(&mut self, mode: Pull) {
        unsafe {
            driverlib::IOCPinTypeGpioInput(self.pin.pin());
        }
        self.set_floating_state(mode);
    }

    fn make_output(&mut self) {
        unsafe {
            driverlib::IOCPinTypeGpioOutput(self.pin.pin());
        }
    }

    fn set_high(&self) {
        GPIO.doutset31_0.write(|w| unsafe { w.bits(self.pin_mask()) });
    }

    fn set_low(&self) {
        GPIO.doutclr31_0.write(|w| unsafe { w.bits(self.pin_mask()) });
    }

    fn is_set_high(&self) -> bool {
        GPIO.dout31_0.read().bits() & self.pin_mask() != 0
    }

    fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    fn is_high(&self) -> bool {
        GPIO.din31_0.read().bits() & self.pin_mask() != 0
    }

    fn is_low(&self) -> bool {
        !self.is_high()
    }

    fn set_floating_state(&self, mode: Pull) {
        let mode = match mode {
            Pull::Down => driverlib::IOC_IOPULL_DOWN,
            Pull::Up => driverlib::IOC_IOPULL_UP,
            Pull::None => driverlib::IOC_NO_IOPULL,
        };

        unsafe { driverlib::IOCIOPortPullSet(self.pin.pin(), mode) }
    }
}

/// Type-erased GPIO pin
pub struct AnyPin {
    pub(crate) pin_port: u32,
}

impl_peripheral!(AnyPin);
impl Pin for AnyPin {}
impl SealedPin for AnyPin {
    #[inline]
    fn pin_port(&self) -> u32 {
        self.pin_port
    }
}

macro_rules! impl_pin {
    ($type:ident, $pin_num:expr) => {
        impl crate::gpio::Pin for peripherals::$type {}
        impl crate::gpio::SealedPin for peripherals::$type {
            #[inline]
            fn pin_port(&self) -> u32 {
                $pin_num
            }
        }

        impl From<peripherals::$type> for crate::gpio::AnyPin {
            fn from(_val: peripherals::$type) -> Self {
                Self { pin_port: $pin_num }
            }
        }
    };
}
pub(crate) use impl_pin;
