use core::cell::Cell;
use core::cell::RefCell;
use core::task::Waker;

use crate::chip::interrupt;
use crate::driverlib;
use crate::pac;

use critical_section::{CriticalSection, Mutex};
use embassy_hal_internal::interrupt::InterruptExt;
use embassy_time_driver::Driver;
use embassy_time_queue_utils::Queue;

// ti-cc2650 has a 70-bit RTC timer, with 64 bits accessible.
// This gives 32 bits for seconds, and 32 bits for "subseconds".
// However, the compare register we use to generate events
// has only 32 bits (16:16). So, the idea is that if we want
// to wait for longer than will fit in this compare register,
// we set it to u32::MAX. Then, executing SYNC after wakeup
// in the interrupt handler should overflow the part against
// which the compare register does the comparison (16 lower bits of secs:16 upper bits of subsecs).
// Then, if the time until we want to sleep will "fit" in the current timeframe
// (we compare it with the full 64bit time), we set the compare register to the expected value.
// Otherwise, we set the compare register to u32::MAX again.
//
// The above is the general idea; with embassy-time, we have to account for
// scheduling loop. Luckily, this is simple as it's just a matter
// of updating the compare register with the new value.

// In hz.
const CLOCK_FREQUENCY: u64 = 32768;

fn rtc() -> pac::AON_RTC::AON_RTC {
    pac::AON_RTC
}

#[inline]
fn combine_time(secs: u32, subsecs: u32) -> u64 {
    (secs as u64) * CLOCK_FREQUENCY + ((subsecs as u64) * CLOCK_FREQUENCY) / (u32::MAX as u64)
}

#[inline]
fn decombine_time(timestamp: u64) -> (u32, u32) {
    (
        (timestamp / CLOCK_FREQUENCY) as u32,
        ((timestamp % CLOCK_FREQUENCY) * (u32::MAX as u64) / CLOCK_FREQUENCY) as u32,
    )
}

/// Used to store the information about the next closest wake up event.
#[derive(Clone, Copy)]
pub(crate) struct Deadline {
    pub(crate) secs: u32,
    pub(crate) subsecs: u32,
}

struct RtcTimeDriver {
    queue: Mutex<RefCell<Queue>>,
    next_deadline: Mutex<Cell<Deadline>>,
}

impl RtcTimeDriver {
    fn init(&'static self) {
        unsafe {
            let interrupts_disabled = driverlib::IntMasterDisable();
            driverlib::AONRTCDisable();
            // Setup wake-up (WU) event
            driverlib::AONRTCEventClear(driverlib::AON_RTC_CH0);
            driverlib::AONEventMcuWakeUpSet(driverlib::AON_EVENT_MCU_WU0, driverlib::AON_EVENT_RTC_CH0);
            driverlib::AONRTCCombinedEventConfig(driverlib::AON_RTC_CH0);

            rtc().SEC().write(|w| w.set_VALUE(0));
            rtc().SUBSEC().write(|w| w.set_VALUE(0));

            driverlib::AONRTCEnable();

            if !interrupts_disabled {
                driverlib::IntMasterEnable();
            }
        }

        interrupt::AON_RTC.unpend();
        unsafe { interrupt::AON_RTC.enable() };
    }

    fn on_interrupt(&self) {
        // clear inter... I mean, event flag.
        unsafe {
            driverlib::AONRTCEventClear(driverlib::AON_RTC_CH0);
        };

        // This will wait for at least one 32khz tick.
        // Add u32::MAX to that and we should overflow
        rtc().SYNC().read();

        critical_section::with(|cs| {
            let next_deadline = self.next_deadline.borrow(cs).get();

            let curr_time = self.now();
            let (curr_secs, _) = decombine_time(curr_time);
            let combined_deadline = combine_time(next_deadline.secs, next_deadline.subsecs);

            if curr_time > combined_deadline {
                self.next_deadline.borrow(cs).set(Deadline { secs: 0, subsecs: 0 });
                let mut next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
                while !self.set_alarm(&cs, next) {
                    next = self.queue.borrow(cs).borrow_mut().next_expiration(self.now());
                }
            } else if next_deadline.secs < curr_secs + 0x10000 {
                // There won't be any more overflows
                let new_time = (next_deadline.secs << 16) | (next_deadline.subsecs >> 16);
                unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, new_time) };
            } else {
                unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, u32::MAX) };
            }
        })
    }

    fn set_alarm(&self, cs: &CriticalSection, at: u64) -> bool {
        let curr_time = self.now();
        // TI-TRM 14.2.3.1
        if at <= curr_time + 4 {
            // In theory, cc2650 RTC should fire an event for timestamps
            // that are at most 1 second past. However, there is no benefit from scheduling "past" events.
            // It's better to cancel "past" events as the events that are now "future" might become "past"
            // by the time we exit critical section.
            unsafe {
                driverlib::AONRTCChannelDisable(driverlib::AON_RTC_CH0);
            };
            return false;
        }

        unsafe {
            driverlib::AONRTCChannelEnable(driverlib::AON_RTC_CH0);
        }

        let (new_secs, new_subsecs) = decombine_time(at);
        let new_deadline = Deadline {
            secs: new_secs,
            subsecs: new_subsecs,
        };
        self.next_deadline.borrow(*cs).set(new_deadline);

        let (curr_secs, _) = decombine_time(curr_time);

        if new_secs.wrapping_sub(curr_secs) >= 0x10000 || (curr_secs & 0xFFFF) > (new_secs & 0xFFFF) {
            unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, u32::MAX) };
        } else {
            let new_time = (new_secs << 16) | (new_subsecs >> 16);
            unsafe { driverlib::AONRTCCompareValueSet(driverlib::AON_RTC_CH0, new_time) };
        }

        return true;
    }
}

impl Driver for RtcTimeDriver {
    fn now(&self) -> u64 {
        let (secs, subsecs) =
            critical_section::with(|_cs| unsafe { (driverlib::AONRTCSecGet(), driverlib::AONRTCFractionGet()) });
        combine_time(secs, subsecs)
    }

    fn schedule_wake(&self, at: u64, waker: &Waker) {
        critical_section::with(|cs| {
            let mut queue = self.queue.borrow(cs).borrow_mut();
            if queue.schedule_wake(at, waker) {
                let mut next = queue.next_expiration(self.now());
                while !self.set_alarm(&cs, next) {
                    next = queue.next_expiration(self.now());
                }
            }
        });
    }
}

embassy_time_driver::time_driver_impl!(static DRIVER: RtcTimeDriver = RtcTimeDriver {
    queue: Mutex::new(RefCell::new(Queue::new())),
    next_deadline: Mutex::new(Cell::new(Deadline {secs: 0, subsecs: 0})),
});

pub(crate) fn init() {
    DRIVER.init()
}

#[interrupt]
fn AON_RTC() {
    DRIVER.on_interrupt()
}
