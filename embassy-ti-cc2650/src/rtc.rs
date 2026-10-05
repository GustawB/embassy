#![macro_use]

use core::cell::UnsafeCell;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::Ordering;
use core::sync::atomic::compiler_fence;
use core::task::Poll;

use crate::chip::interrupt;
use crate::chip::interrupt::typelevel::Interrupt;
use crate::define_peri;
use crate::driverlib;
use crate::pac;
use crate::time_driver::deadline_out_of_range;
use embassy_hal_internal::drop::OnDrop;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use paste::paste;

// Simple RTC driver that can be used if embassy-time is not enabled only.
// It is advised to use embassy-time though.
// For a general description see time_driver.rs, logic is almost the same.

define_peri!(AonRtc, aon_rtc, 0x40092000);

/// Interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

#[inline]
fn get_curr_time() -> (u32, u32) {
    critical_section::with(|_cs| unsafe { (driverlib::AONRTCSecGet(), driverlib::AONRTCFractionGet()) })
}

#[inline]
fn combine_time(secs: u32, subsecs: u32) -> u64 {
    ((secs as u64) << 32) | (subsecs as u64)
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let s = T::state();

        // clear inter... I mean, event flag.
        unsafe {
            driverlib::AONRTCEventClear(driverlib::AON_RTC_CH0);
        };

        AON_RTC.sync.read().bits();

        let next_deadline = s
            .get_next_deadline()
            .unwrap_or_else(|| Deadline { secs: 0, subsecs: 0 });
        let (curr_secs, curr_subsecs) = get_curr_time();
        let combined_deadline = combine_time(next_deadline.secs, next_deadline.subsecs);
        let combined_time = combine_time(curr_secs, curr_subsecs);

        if combined_time > combined_deadline {
            unsafe {
                driverlib::AONRTCChannelDisable(driverlib::AON_RTC_CH0);
            };
            s.clear_next_deadline();
            s.rtc_waker.wake();
        } else if !deadline_out_of_range(curr_secs, next_deadline.secs) {
            // There won't be any more overflows
            let new_time = (next_deadline.secs << 16) | (next_deadline.subsecs >> 16);
            unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, new_time) };
        } else {
            unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, u32::MAX) };
        }
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

#[cfg(not(feature = "time-driver"))]
macro_rules! impl_rtc {
    ($type:ident, $irq:ident) => {
        impl crate::rtc::SealedInstance for peripherals::$type {
            fn state() -> &'static crate::rtc::State {
                static STATE: crate::rtc::State = crate::rtc::State::new();
                &STATE
            }
        }
        impl crate::rtc::Instance for peripherals::$type {
            type Interrupt = crate::interrupt::typelevel::$irq;
        }
    };
}

#[derive(Clone, Copy)]
pub(crate) struct Deadline {
    pub(crate) secs: u32,
    pub(crate) subsecs: u32,
}

pub(crate) struct State {
    pub(crate) rtc_waker: AtomicWaker,
    next_deadline: UnsafeCell<Option<Deadline>>,
}

// SAFETY: State is used only in:
// 1. current thread, if there are no sleeps scheduled
// 2. In the irq handler, with the guarantee that there is no
// ongoing update from the current thread.
unsafe impl Sync for State {}

impl State {
    #[cfg(not(feature = "time-driver"))]
    pub(crate) const fn new() -> Self {
        Self {
            rtc_waker: AtomicWaker::new(),
            next_deadline: UnsafeCell::new(None),
        }
    }

    pub(crate) fn set_next_deadline(&self, deadline: Deadline) {
        unsafe {
            (*self.next_deadline.get()) = Some(deadline);
        };
    }

    pub(crate) fn clear_next_deadline(&self) {
        unsafe {
            (*self.next_deadline.get()) = None;
        };
    }
    pub(crate) fn get_next_deadline(&self) -> Option<Deadline> {
        unsafe { *self.next_deadline.get() }
    }
}

pub struct Rtc<'a, T: Instance> {
    state: &'static State,
    _peri: Peri<'a, T>,
}

impl<'a, T: Instance> Rtc<'a, T> {
    pub fn new(
        aon_rtc: Peri<'a, T>,
        _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
    ) -> Self {
        unsafe {
            let interrupts_disabled = driverlib::IntMasterDisable();
            driverlib::AONRTCDisable();
            // Setup wake-up (WU) event
            driverlib::AONRTCEventClear(driverlib::AON_RTC_CH0);
            driverlib::AONEventMcuWakeUpSet(driverlib::AON_EVENT_MCU_WU0, driverlib::AON_EVENT_RTC_CH0);
            driverlib::AONRTCCombinedEventConfig(driverlib::AON_RTC_CH0);

            AON_RTC.sec.reset();
            AON_RTC.subsec.reset();

            driverlib::AONRTCEnable();

            if !interrupts_disabled {
                driverlib::IntMasterEnable();
            }
        }

        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };

        Rtc {
            state: T::state(),
            _peri: aon_rtc,
        }
    }

    async fn internal_sleep(&self, curr_secs: u32, next_secs: u32, next_subsecs: u32) {
        let next_deadline = Deadline {
            secs: next_secs,
            subsecs: next_subsecs,
        };

        self.state.set_next_deadline(next_deadline);

        compiler_fence(Ordering::SeqCst);

        unsafe {
            driverlib::AONRTCChannelEnable(driverlib::AON_RTC_CH0);
        };

        if deadline_out_of_range(curr_secs, next_secs) {
            unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, u32::MAX) };
        } else {
            let new_time = (next_secs << 16) | (next_subsecs >> 16);
            unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, new_time) };
        }

        let drop = OnDrop::new(move || {
            critical_section::with(|_cs| {
                unsafe {
                    driverlib::AONRTCChannelDisable(driverlib::AON_RTC_CH0);
                    self.state.clear_next_deadline();
                    driverlib::AONRTCEventClear(driverlib::AON_RTC_CH0);
                };
            })
        });

        let combined_next_time = combine_time(next_secs, next_subsecs);
        let _ = poll_fn(|cx| {
            self.state.rtc_waker.register(cx.waker());
            let (new_secs, new_subsecs) = get_curr_time();
            let combined_new_time = combine_time(new_secs, new_subsecs);
            if combined_next_time <= combined_new_time {
                return Poll::Ready(());
            }
            Poll::Pending
        })
        .await;
        drop.defuse();
    }

    /// Sleeps for the specified amount of time in seconds.
    pub async fn sleep(&mut self, seconds: u32) {
        if seconds == 0 {
            return;
        }

        let (curr_secs, curr_subsecs) = get_curr_time();
        // 2^32 seconds is around 130 years, so if this overflows, we probably want that panic.
        let next_secs = curr_secs + seconds as u32;

        self.internal_sleep(curr_secs, next_secs, curr_subsecs).await;
    }

    /// Sleeps for the specified amount of time in milliseconds.
    pub async fn sleep_millis(&mut self, milliseconds: u32) {
        if milliseconds == 0 {
            return;
        }

        let (curr_secs, curr_subsecs) = get_curr_time();
        let mut next_secs = curr_secs + (milliseconds / 1000);
        let next_subsecs = curr_subsecs.wrapping_add(((milliseconds % 1000) as u64 * (1u64 << 32) / 1000) as u32);
        if next_subsecs < curr_subsecs {
            // Overflow
            next_secs += 1;
        }

        self.internal_sleep(curr_secs, next_secs, next_subsecs).await;
    }

    /// Wake up at the specified time.
    /// Time starts at zero from boot.
    pub async fn wakeup_at(&mut self, seconds: u32, milliseconds: u32) {
        // Same as in sleep(): overflowing 2^32 seconds deserves a panic.
        let seconds = seconds + milliseconds / 1000;
        let (curr_secs, curr_subsecs) = get_curr_time();
        let combined_curr_time = combine_time(curr_secs, curr_subsecs);
        let next_subsecs = ((milliseconds % 1000) as u64 * (1u64 << 32) / 1000) as u32;
        let combined_new_time = combine_time(seconds, next_subsecs);
        if combined_curr_time >= combined_new_time {
            return;
        }

        self.internal_sleep(curr_secs, seconds, next_subsecs).await;
    }
}

#[cfg(not(feature = "time-driver"))]
pub(crate) use impl_rtc;
