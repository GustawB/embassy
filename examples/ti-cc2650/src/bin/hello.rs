#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650;
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    loop {}
}
