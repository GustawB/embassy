#![macro_use]

use crate::chip::interrupt;
use crate::chip::interrupt::typelevel::Interrupt;
use crate::driverlib;
use crate::pac;
use core::cell::UnsafeCell;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;
use core::sync::atomic::compiler_fence;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use ti_cc2650_pac::GPT0::vals;

const CLOCK_FREQUENCY: u64 = 48000000;

/// Interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let r = T::regs();
        let s = T::state();
        let irq_mask = r.MIS().read();

        r.ICLR().modify(|w| {
            w.set_TATOCINT(true);
            w.set_TAMCINT(true);
        });

        // SAFETY: interrupts are enabled ONLY IF transaction is set.
        let mut st = s.get_curr_transaction().unwrap();

        if irq_mask.TATOMIS() {
            // Overflow happened, update overflow count and check it we met the limit.
            st.overflow_count += 1;
            s.set_new_transaction(st);
            if st.overflow_count >= st.overflow_limit {
                let curr_time = unsafe { driverlib::TimerValueGet(driverlib::GPT0_BASE, driverlib::TIMER_A) };
                if curr_time >= st.final_deadline {
                    unsafe {
                        driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);
                    };
                    s.transaction_finished.store(true, Ordering::Release);
                    s.gpt_waker.wake();
                    return;
                }

                unsafe {
                    driverlib::TimerMatchSet(driverlib::GPT0_BASE, driverlib::TIMER_A, st.final_deadline);
                };
                r.TAMR().modify(|w| w.set_TAMIE(vals::TAMIE::EN));
            } else {
                unsafe {
                    driverlib::TimerMatchSet(driverlib::GPT0_BASE, driverlib::TIMER_A, u32::MAX);
                };
            }
        }

        if irq_mask.TAMMIS() {
            // Match interrupt fired, we finished the sleep transaction.
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
    fn regs() -> pac::GPT0::GPT0;
    fn state() -> &'static State;
}

/// AON_RTC peripheral instance.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    /// Interrupt for this peripheral.
    type Interrupt: interrupt::typelevel::Interrupt;
}

macro_rules! impl_gpt {
    ($type:ident, $pac_type:ident, $irq:ident) => {
        impl crate::gpt::SealedInstance for peripherals::$type {
            fn regs() -> pac::GPT0::GPT0 {
                pac::$pac_type
            }
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
    pub(crate) final_deadline: u32,
    pub(crate) overflow_limit: u32,
    pub(crate) overflow_count: u32,
}

impl SleepTransaction {
    pub(crate) fn new(final_deadline: u32, overflow_limit: u32) -> Self {
        Self {
            final_deadline,
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
    r: pac::GPT0::GPT0,
    state: &'static State,
    _peri: Peri<'a, T>,
}

impl<'a, T: Instance> Gpt<'a, T> {
    pub fn new(
        gpt: Peri<'a, T>,
        _irq: impl interrupt::typelevel::Binding<T::Interrupt, InterruptHandler<T>> + 'a,
    ) -> Self {
        let r = T::regs();
        unsafe {
            driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);
            r.CFG().modify(|w| w.set_CFG(vals::CFG::_32BIT_TIMER));

            r.TAMR().modify(|w| {
                w.set_TACINTD(vals::TACINTD::EN_TO_INTR); // Enable time-out event interrupts.
                w.set_TAMIE(vals::TAMIE::EN); // Enable match interrupts.
                w.set_TACDIR(vals::TACDIR::UP); // Count up.
                w.set_TAMR(vals::TAMR::PERIODIC); // Run in periodic mode.
            });

            // Stop GPT when debugger halts the program.
            r.CTL().modify(|w| w.set_TASTALL(vals::TASTALL::EN));

            // Enable mathc and time-out interrupts.
            r.IMR().modify(|w| w.set_TAMIM(vals::TAMIM::EN));
            r.IMR().modify(|w| w.set_TATOIM(vals::TATOIM::EN));
        };

        T::Interrupt::unpend();
        unsafe { T::Interrupt::enable() };

        Gpt {
            r: T::regs(),
            state: T::state(),
            _peri: gpt,
        }
    }

    async fn sleep_internal(&self, st: SleepTransaction) {
        unsafe {
            driverlib::TimerDisable(driverlib::GPT0_BASE, driverlib::TIMER_A);
            self.r.TAV().write(|w| w.set_TAV(0));

            //driverlib::TimerLoadSet(driverlib::GPT0_BASE, driverlib::TIMER_A);
            if st.overflow_limit == 0 {
                // No overflow, we immediately want to seel for the specified amount.
                driverlib::TimerMatchSet(driverlib::GPT0_BASE, driverlib::TIMER_A, st.final_deadline);
                self.r.TAMR().modify(|w| w.set_TAMIE(vals::TAMIE::EN));
            } else {
                // There is overflow, so we first need to sleep for u32::MAX for
                // SleepTransaction::overflow_limit times.
                driverlib::TimerMatchSet(driverlib::GPT0_BASE, driverlib::TIMER_A, u32::MAX);
                self.r.TAMR().modify(|w| w.set_TAMIE(vals::TAMIE::DIS));
            }
            self.state.set_new_transaction(st);

            compiler_fence(Ordering::SeqCst);
            driverlib::TimerEnable(driverlib::GPT0_BASE, driverlib::TIMER_A);
        };

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
    }

    /// Sleeps for the specified amount of time in seconds.
    pub async fn sleep(&mut self, seconds: u32) {
        let sleep_in_hz = (seconds as u64) * CLOCK_FREQUENCY;
        let st = SleepTransaction::new(
            (sleep_in_hz % (u32::MAX as u64)) as u32,
            (sleep_in_hz / (u32::MAX as u64)) as u32,
        );
        self.sleep_internal(st).await;
    }

    /// Sleeps for the specified amount of time in milliseconds.
    pub async fn sleep_millis(&mut self, milliseconds: u32) {
        let sleep_in_hz = (milliseconds as u64) * CLOCK_FREQUENCY / 1000;
        let st = SleepTransaction::new(
            (sleep_in_hz % (u32::MAX as u64)) as u32,
            (sleep_in_hz / (u32::MAX as u64)) as u32,
        );
        self.sleep_internal(st).await;
    }
}

pub(crate) use impl_gpt;
