#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::chip::peripherals;
use embassy_ti_cc2650::debug_print::debug_print;
use embassy_ti_cc2650::{bind_interrupts, uart};
use embassy_time::Timer;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

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
    let _ = embassy_ti_cc2650::init();
    spawner.spawn(slow_task().unwrap());
    spawner.spawn(fast_task().unwrap());
}
