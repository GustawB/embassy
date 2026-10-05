#![macro_use]

use crate::chip::interrupt;
use crate::chip::interrupt::typelevel::Interrupt;
use crate::define_peri;
use crate::driverlib;
use crate::pac;
use core::cell::UnsafeCell;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use core::sync::atomic::compiler_fence;
use core::task::Poll;
use embassy_hal_internal::drop::OnDrop;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use paste::paste;

define_peri!(Gpt0, gpt0, 0x40010000);

const OVERFLOW_CYCLES: u64 = 1u64 << 32;

/// Interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let s = T::state();
        let irq_mask = GPT0.mis.read();

        unsafe {
            GPT0.iclr.write(|w| w.bits(irq_mask.bits()));
        }

        let Some(mut st) = s.get_curr_transaction() else { return };

        // Match register holds deadline for the whole sleep, so the match
        // fires once per lap. We never touch it here, so there is no
        // read-then-set race with the running counter.
        // Overflow is handled first, so a timeout and a match landing in the
        // same IRQ are counted in the right order.
        if irq_mask.tatomis().bit_is_set() {
            // Overflow happened, update overflow count.
            st.overflow_count += 1;
            s.set_new_transaction(st);
        }

        // Match and overflow in the same IRQ: if the counter is still below the deadline,
        // the match fired before the overflow, i.e. in the previous lap.
        let stale_match = irq_mask.tatomis().bit_is_set()
            && unsafe { driverlib::TimerValueGet(driverlib::GPT0_BASE, driverlib::TIMER_A) } < st.deadline;

        if (irq_mask.tammis().bit_is_set() && !stale_match && st.overflow_count == st.overflow_limit)
            || st.overflow_count > st.overflow_limit
        {
            s.clear_transaction();
            unsafe {
                driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);
            };
            s.transaction_finished.store(true, Ordering::Release);
            s.gpt_waker.wake();
        }
    }
}

pub(crate) trait SealedInstance {
    fn state() -> &'static State;
}

/// GPT peripheral instance.
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

#[derive(Clone, Copy)]
pub(crate) struct SleepTransaction {
    pub(crate) deadline: u32,
    pub(crate) overflow_limit: u32,
    pub(crate) overflow_count: u32,
}

impl SleepTransaction {
    pub(crate) fn new(deadline: u32, overflow_limit: u32) -> Self {
        Self {
            deadline,
            overflow_limit,
            overflow_count: 0,
        }
    }
}

// SAFETY: State is used only in:
// 1. current thread, if there are no sleeps scheduled
// 2. In the irq handler, with the guarantee that there is no
// ongoing update from the current thread.
pub(crate) struct State {
    pub(crate) gpt_waker: AtomicWaker,
    pub(crate) transaction_finished: AtomicBool,
    curr_transaction: UnsafeCell<Option<SleepTransaction>>,
}

unsafe impl Sync for State {}

impl State {
    pub(crate) const fn new() -> Self {
        Self {
            gpt_waker: AtomicWaker::new(),
            transaction_finished: AtomicBool::new(false),
            curr_transaction: UnsafeCell::new(None),
        }
    }

    pub(crate) fn set_new_transaction(&self, transaction: SleepTransaction) {
        unsafe {
            (*self.curr_transaction.get()) = Some(transaction);
        };
    }

    pub(crate) fn clear_transaction(&self) {
        unsafe {
            (*self.curr_transaction.get()) = None;
        };
    }

    pub(crate) fn get_curr_transaction(&self) -> Option<SleepTransaction> {
        unsafe { *self.curr_transaction.get() }
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
                    .en_to_intr() // Enable time-out event interrupts.
                    .tamie()
                    .en() // Enable match interrupts.
                    .tacdir()
                    .up() // Count up.
                    .tamr()
                    .periodic() // Run in periodic mode.
            });

            // Stop GPT when debugger halts the program.
            GPT0.ctl.write(|w| w.tastall().set_bit());

            // Enable match and time-out interrupts.
            GPT0.imr.modify(|_r, w| w.tamim().en().tatoim().en());
        };

        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };

        Gpt {
            state: T::state(),
            _peri: gpt,
        }
    }

    async fn sleep_internal(&self, st: SleepTransaction) {
        // The counter starts at 0, so a zero deadline might never produce a match.
        if st.deadline == 0 && st.overflow_limit == 0 {
            return;
        }

        unsafe {
            driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);

            // Disabling the timer as the first op is crucial for e.g. timeout counting.
            compiler_fence(Ordering::SeqCst);

            GPT0.tav.reset();

            // Match stays at deadline for the whole sleep; the IRQ handler
            // ignores matches until SleepTransaction::overflow_limit laps have passed.
            driverlib::TimerMatchSet(driverlib::GPT0_BASE, driverlib::TIMER_A, st.deadline);
            GPT0.tamr.modify(|_r, w| w.tamie().en());
            self.state.set_new_transaction(st);

            compiler_fence(Ordering::SeqCst);
            driverlib::TimerEnable(driverlib::GPT0_BASE, driverlib::TIMER_A);
        };

        let drop = OnDrop::new(move || {
            critical_section::with(|_cs| {
                unsafe {
                    driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);
                    self.state.clear_transaction();
                    self.state.transaction_finished.store(false, Ordering::SeqCst);
                    GPT0.iclr.modify(|_r, w| w.bits(u32::MAX));
                };
            })
        });

        let _ = poll_fn(|cx| {
            self.state.gpt_waker.register(cx.waker());
            match self
                .state
                .transaction_finished
                .compare_exchange(true, false, Ordering::Acquire, Ordering::SeqCst)
            {
                Ok(_) => Poll::Ready(()),
                Err(_) => Poll::Pending,
            }
        })
        .await;
        drop.defuse();
    }

    /// Sleeps for the specified amount of time in seconds.
    pub async fn sleep(&mut self, seconds: u32) {
        let clock_frequency = unsafe { driverlib::SysCtrlClockGet() } as u64;
        let sleep_in_hz = (seconds as u64) * clock_frequency;
        let st = SleepTransaction::new(
            (sleep_in_hz % OVERFLOW_CYCLES) as u32,
            (sleep_in_hz / OVERFLOW_CYCLES) as u32,
        );
        self.sleep_internal(st).await;
    }

    /// Sleeps for the specified amount of time in milliseconds.
    pub async fn sleep_millis(&mut self, milliseconds: u32) {
        let clock_frequency = unsafe { driverlib::SysCtrlClockGet() } as u64;
        let sleep_in_hz = (milliseconds as u64) * clock_frequency / 1000;
        let st = SleepTransaction::new(
            (sleep_in_hz % OVERFLOW_CYCLES) as u32,
            (sleep_in_hz / OVERFLOW_CYCLES) as u32,
        );
        self.sleep_internal(st).await;
    }
}

pub(crate) use impl_gpt;
