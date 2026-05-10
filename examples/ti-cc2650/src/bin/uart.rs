#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::chip::peripherals;
use embassy_ti_cc2650::uart::{Config, UartFull};
use embassy_ti_cc2650::{bind_interrupts, uart};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    UART0 => uart::InterruptHandler<peripherals::UART0>;
    //UDMA => uart::InterruptHandler<peripherals::UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let config = Config::default();
    let uart = UartFull::new(p.UART0, config, Irqs);

    let mut buf = [0; 8];
    buf.copy_from_slice(b"Hello9\r\n");
    loop {
        let _ = uart.write(&buf, 8).await;
    }
}
