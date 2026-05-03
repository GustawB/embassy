#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::gpio::{Level, Output};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let mut led = Output::new(p.P_13, Level::Low);

    led.set_high();
    led.set_low();
    led.set_high();
    led.set_low();
    led.set_high();
    led.set_low();
    loop {}
}
