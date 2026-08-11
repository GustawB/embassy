#![macro_use]

use crate::chip::interrupt;
use crate::chip::interrupt::typelevel::Interrupt;
use crate::define_peri;
use crate::driverlib;
use crate::pac;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::Ordering;
use core::sync::atomic::compiler_fence;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use paste::paste;

// 1073807360 is the start address of registers for GPT0.
// cc2650 crate calls it RegisterBlock; I took this
// addres from said crate.
define_peri!(Gpt0, gpt0, 1073807360);

const CLOCK_FREQUENCY: u32 = 48000000;

/// Interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let s = T::state();

        GPT0.iclr.modify(|_r, w| w.tamcint().set_bit());
        unsafe {
            driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);
        }

        s.gpt_waker.wake();
    }
}

pub(crate) trait SealedInstance {
    fn state() -> &'static State;
}

/// AON_RTC peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    /// Interrupt for this peripheral.
    type Interrupt: interrupt::typelevel::Interrupt;
}

macro_rules! impl_gpt {
    ($type:ident, $irq:ident) => {
        impl crate::gpt::SealedInstance for peripherals::$type {
            fn state() -> &'static crate::gpt::State {
                static STATE: crate::gpt::State = crate::gpt::State::new();
                &STATE
            }
        }
        impl crate::gpt::Instance for peripherals::$type {
            type Interrupt = crate::interrupt::typelevel::$irq;
        }
    };
}

pub(crate) struct State {
    pub(crate) gpt_waker: AtomicWaker,
}

impl State {
    pub(crate) const fn new() -> Self {
        Self {
            gpt_waker: AtomicWaker::new(),
        }
    }
}

pub struct Gpt<'a, T: Instance> {
    state: &'static State,
    _peri: Peri<'a, T>,
}

impl<'a, T: Instance> Gpt<'a, T> {
    pub fn new(
        gpt: Peri<'a, T>,
        _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
    ) -> Self {
        unsafe {
            driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);
            GPT0.cfg.write(|w| w.cfg()._32bit_timer());

            GPT0.tamr.modify(|_r, w| {
                w.tacintd()
                    .dis_to_intr() // Disable time-out event interrupts.
                    .tamie()
                    .en() // Enable match interrupts.
                    .tacdir()
                    .up() // Count up.
                    .tamr()
                    .periodic() // Run in periodic mode.
            });

            // Stop GPT when debugger halts the program.
            GPT0.ctl.write(|w| w.tastall().set_bit());

            // Enable match interrupt
            GPT0.imr.modify(|_r, w| w.tamim().en());
        };

        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };

        Gpt {
            state: T::state(),
            _peri: gpt,
        }
    }

    async fn sleep_internal(&self, sleep_in_hz: u32) {
        unsafe {
            driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);
            GPT0.tav.reset();

            //driverlib::TimerLoadSet(driverlib::GPT0_BASE, driverlib::TIMER_A);
            driverlib::TimerMatchSet(driverlib::GPT0_BASE, driverlib::TIMER_A, sleep_in_hz);

            compiler_fence(Ordering::SeqCst);
            driverlib::TimerEnable(driverlib::GPT0_BASE, driverlib::TIMER_A);
        };

        let _ = poll_fn(|cx| {
            self.state.gpt_waker.register(cx.waker());
            let curr_time = unsafe { driverlib::TimerValueGet(driverlib::GPT0_BASE, driverlib::TIMER_A) };
            if curr_time >= sleep_in_hz {
                return Poll::Ready(());
            }
            return Poll::Pending;
        })
        .await;
    }

    /// Sleeps for the specified amount of time in seconds.
    pub async fn sleep(&mut self, seconds: u32) {
        let sleep_in_hz = seconds * CLOCK_FREQUENCY;
        self.sleep_internal(sleep_in_hz).await;
    }

    /// Sleeps for the specified amount of time in milliseconds.
    pub async fn sleep_millis(&mut self, milliseconds: u32) {
        let sleep_in_hz = milliseconds * CLOCK_FREQUENCY / 1000;
        self.sleep_internal(sleep_in_hz).await;
    }
}

pub(crate) use impl_gpt;
