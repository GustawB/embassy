#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::debug_print::{DebugPrint, debug_print};
use embassy_time::Timer;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let _debug_print = DebugPrint::new(p.P_28, p.P_29);

    let buf = [b'A'; 1800];
    loop {
        debug_print(&buf);
        Timer::after_secs(5).await;
    }
}
