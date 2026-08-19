#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::chip::peripherals;
use embassy_ti_cc2650::uart::{Config, UartFull};
use embassy_ti_cc2650::{bind_interrupts, uart};
use embassy_time::Timer;
use panic_probe as _;
use static_cell::StaticCell;

const BAUDRATE: u32 = 115200;
const UART_COMM_BUF_SIZE: usize = 1024;

bind_interrupts!(struct Irqs {
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let mut config = Config::default();
    config.baudrate = BAUDRATE;

    static COMM_BUF: StaticCell<[u16; UART_COMM_BUF_SIZE]> = StaticCell::new();
    let comm_buf = COMM_BUF.init([0u16; UART_COMM_BUF_SIZE]);

    let (mut uart, _) = UartFull::new(p.UART0, config, comm_buf, Irqs);

    let buf = "Hello world\n".as_bytes();
    loop {
        uart.write(buf).await.unwrap();
        Timer::after_millis(750).await;
    }
}
