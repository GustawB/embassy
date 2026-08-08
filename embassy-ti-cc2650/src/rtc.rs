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
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use paste::paste;

// 1074339840 is the start address of registers for AON_RTC.
// cc2650 crate calls it RegisterBlock; I took this
// addres from said crate.
define_peri!(Aon_rtc, aon_rtc, 1074339840);

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

        // This will wait for at least one 32khz tick.
        // Add u32::MAX to that and we should overflow
        AON_RTC.sync.read().bits();

        let next_deadline = s
            .get_next_deadline()
            .unwrap_or_else(|| Deadline { secs: 0, subsecs: 0 });
        let (curr_secs, curr_subsecs) = get_curr_time();
        let combined_deadline = combine_time(next_deadline.secs, next_deadline.subsecs);
        let combined_time = combine_time(curr_secs, curr_subsecs);

        if combined_time > combined_deadline {
            s.clear_next_deadline();
            s.rtc_waker.wake();
        } else if next_deadline.secs < curr_secs + 0x10000 {
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
            // Setup wake-up (WU) events
            driverlib::AONRTCEventClear(driverlib::AON_RTC_CH0);
            driverlib::AONRTCEventClear(driverlib::AON_RTC_CH1);
            driverlib::AONRTCEventClear(driverlib::AON_RTC_CH2);
            driverlib::AONEventMcuWakeUpSet(driverlib::AON_EVENT_MCU_WU0, driverlib::AON_EVENT_RTC_CH0);
            driverlib::AONEventMcuWakeUpSet(driverlib::AON_EVENT_MCU_WU1, driverlib::AON_EVENT_RTC_CH1);
            driverlib::AONEventMcuWakeUpSet(driverlib::AON_EVENT_MCU_WU2, driverlib::AON_EVENT_RTC_CH2);
            driverlib::AONRTCCombinedEventConfig(
                driverlib::AON_RTC_CH0, // | driverlib::AON_RTC_CH1 | driverlib::AON_RTC_CH2,
            );

            AON_RTC.sec.reset();
            AON_RTC.subsec.reset();

            // Enable event channel 0
            driverlib::AONRTCChannelEnable(driverlib::AON_RTC_CH0);
            // Enable AON_RTC module
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

        if next_secs.wrapping_sub(curr_secs) >= 0x10000 || (curr_secs & 0xFFFF) > (next_secs & 0xFFFF) {
            unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, u32::MAX) };
        } else {
            let new_time = (next_secs << 16) | (next_subsecs >> 16);
            unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, new_time) };
        }

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
    }

    /// Returns seconds and milliseconds passed since boot.
    #[inline]
    pub fn get_current_time(&self) -> (u32, u32) {
        get_curr_time()
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
        // 1000 ~ 2^10; curr_subsecs are 32bit, so we need to shift by 22
        let next_subsecs = curr_subsecs.wrapping_add((milliseconds % 1000) << 22);
        if next_subsecs < curr_subsecs {
            // Overflow
            next_secs += 1;
        }

        self.internal_sleep(curr_secs, next_secs, next_subsecs).await;
    }

    /// Wake up at the specified time.
    /// Time starts at zero from boot.
    pub async fn wakeup_at(&mut self, seconds: u32, milliseconds: u32) {
        let (curr_secs, curr_subsecs) = get_curr_time();
        let combined_curr_time = combine_time(curr_secs, curr_subsecs);
        let combined_new_time = combine_time(seconds, milliseconds << 5);
        if combined_curr_time >= combined_new_time {
            return;
        }

        self.internal_sleep(curr_secs, seconds, milliseconds).await;
    }
}

pub(crate) use impl_rtc;
