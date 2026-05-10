#![macro_use]

use crate::driverlib;
use embassy_hal_internal::Peri;
use embassy_hal_internal::PeripheralType;
use embassy_hal_internal::impl_peripheral;

mod internals {
    use crate::pac;
    use core::ops::Deref;

    pub(super) struct Gpio(*const pac::gpio::RegisterBlock);
    unsafe impl Send for Gpio {}
    unsafe impl Sync for Gpio {}

    // taken straight from cc2650 crate
    const GPIO_REGISTER_BLOCK_ADDR: usize = 1073881088;
    pub(super) static GPIO: Gpio = Gpio(GPIO_REGISTER_BLOCK_ADDR as *const _);

    impl Deref for Gpio {
        type Target = pac::gpio::RegisterBlock;

        fn deref(&self) -> &Self::Target {
            unsafe { &*self.0 }
        }
    }
}
use internals::GPIO;

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
        let gpio_pin = GPIOPin::new(pin);
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
        let gpio_pin = GPIOPin::new(pin);
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

    /// Toggle the output level.
    #[inline]
    pub fn toggle(&mut self) {
        self.gpio_pin.toggle()
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

pub trait Pin: PeripheralType + Into<AnyPin> + SealedPin + Sized + 'static {
    #[inline]
    fn pin(&self) -> u32 {
        self.pin_port()
    }
}

pub(crate) struct GPIOPin<'d> {
    pin: Peri<'d, AnyPin>,
    pin_mask: u32,
}

impl<'d> GPIOPin<'d> {
    fn new(pin: Peri<'d, impl Pin>) -> Self {
        let any_pin = pin.into();
        let pin_mask = 1 << any_pin.pin();
        Self { pin: any_pin, pin_mask }
    }

    fn make_input(&self, mode: Pull) {
        self.enable_gpio();
        self.enable_input();
        self.set_floating_state(mode);
    }

    fn make_output(&self) {
        self.enable_gpio();
        self.enable_output();
    }

    fn set_high(&self) {
        GPIO.doutset31_0.write(|w| unsafe { w.bits(self.pin_mask) });
    }

    fn set_low(&self) {
        GPIO.doutclr31_0.write(|w| unsafe { w.bits(self.pin_mask) });
    }

    fn toggle(&self) {
        GPIO.douttgl31_0.modify(|_r, w| unsafe { w.bits(self.pin_mask) });
    }

    fn is_set_high(&self) -> bool {
        GPIO.dout31_0.read().bits() & self.pin_mask != 0
    }

    fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    fn is_high(&self) -> bool {
        GPIO.din31_0.read().bits() & self.pin_mask != 0
    }

    fn is_low(&self) -> bool {
        !self.is_high()
    }

    fn enable_gpio(&self) {
        // Driverlib is better here: cc2650 crate requires either matching over 32 options or a lot of unsafe.
        // OTOH both IOCPortConfigure{G,S}et are present in ROM.
        let pin_config = unsafe { driverlib::IOCPortConfigureGet(self.pin.pin()) };
        unsafe { driverlib::IOCPortConfigureSet(self.pin.pin(), driverlib::IOC_PORT_GPIO, pin_config) };
    }

    fn enable_output(&self) {
        //self.set_floating_state(Pull::None);
        // unsafe { driverlib::GPIO_setOutputEnableDio(self.pin, driverlib::GPIO_OUTPUT_ENABLE) };
        GPIO.doe31_0.modify(|_r, w| unsafe { w.bits(self.pin_mask) });
    }

    fn enable_input(&self) {
        // Driverlib is better here: cc2650 crate requires either matching over 32 options or a lot of unsafe.
        // OTOH both IOCPortConfigure{G,S}et are present in ROM.
        let mut pin_config = unsafe { driverlib::IOCPortConfigureGet(self.pin.pin()) };
        pin_config |= driverlib::IOC_INPUT_ENABLE;
        unsafe { driverlib::IOCPortConfigureSet(self.pin.pin(), driverlib::IOC_PORT_GPIO, pin_config) };
    }

    fn set_floating_state(&self, mode: Pull) {
        // Driverlib is better here: IOCIOPortPullSet is present in ROM.
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
