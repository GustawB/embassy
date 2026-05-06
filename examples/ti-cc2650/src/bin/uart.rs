#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::uart::{Config, UartFull};
use panic_probe as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let config = Config::default();
    let mut uart = UartFull::new(p.UART0, config);

    let mut buf = [0; 8];
    buf.copy_from_slice(b"Hello!\r\n");
    loop {
        unsafe {
            uart.write(&buf);
        }
    }
}
