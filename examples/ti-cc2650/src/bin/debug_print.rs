#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::debug_print::{DebugPrint, debug_print};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::task]
async fn slow_task() -> ! {
    let buf = "Slow hello\n".as_bytes();
    loop {
        debug_print(buf);
        Timer::after_millis(1500).await;
    }
}

#[embassy_executor::task]
async fn fast_task() -> ! {
    let buf = "Fast hello\n".as_bytes();
    loop {
        debug_print(buf);
        Timer::after_millis(750).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let _debug_print = DebugPrint::new(p.P_28, p.P_29);
    spawner.spawn(slow_task().unwrap());
    spawner.spawn(fast_task().unwrap());
}
