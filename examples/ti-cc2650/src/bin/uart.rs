#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_ti_cc2650::chip::peripherals;
use embassy_ti_cc2650::uart::{Config, UartFull};
use embassy_ti_cc2650::{bind_interrupts, uart};
use heapless::String;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

const CHUNK_SIZE: usize = 8;
const BAUDRATE: u32 = 115200;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let mut config = Config::default();
    config.baudrate = BAUDRATE;
    let uart = UartFull::new(p.UART0, config, Irqs);

    let mut buf = [0; CHUNK_SIZE];
    let (tx, rx) = uart.split();
    loop {
        // uart.read(&mut buf, CHUNK_SIZE).await.unwrap();
        // uart.write(&buf, CHUNK_SIZE).await.unwrap();
        let bytes_read = rx.read(&mut buf, CHUNK_SIZE).await.unwrap();
        let mut text_buffer: String<16> = String::new();
        write!(text_buffer, "{}\r\n", bytes_read).unwrap();
        tx.write(text_buffer.as_bytes(), text_buffer.len()).await.unwrap();
        //tx.write(&buf, CHUNK_SIZE).await.unwrap();
    }
}
