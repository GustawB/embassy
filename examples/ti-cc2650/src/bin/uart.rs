#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::chip::peripherals;
use embassy_ti_cc2650::uart::{Config, UartFull, UartFullRxReceiver, UartFullRxReceiverImpl, UartFullRxRunner};
use embassy_ti_cc2650::{bind_interrupts, uart};
use panic_probe as _;

bind_interrupts!(struct Irqs {
    UART0 => uart::InterruptHandler<peripherals::UART0>;
});

#[embassy_executor::task]
async fn uart_task(mut runner: UartFullRxRunner<'static>) {
    runner.run().await;
}

const CHUNK_SIZE: usize = 66;
const BAUDRATE: u32 = 115200;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let mut config = Config::default();
    config.baudrate = BAUDRATE;

    let (uart, rx_end) = UartFull::new(p.UART0, config, Irqs);
    let mut uart_receiver = UartFullRxReceiverImpl::new(rx_end);

    // split() consumes uart.
    let (mut tx, rx_runner) = uart.split();
    spawner.spawn(uart_task(rx_runner).unwrap());

    let mut buf = [0; CHUNK_SIZE - 2];
    loop {
        // This will read bytes until it encounters '\n' OR fills the whole buffer.
        let bytes_read = uart_receiver.read(&mut buf).await.unwrap();
        // Receiver ignores '\n', so it might return 0.
        tx.write(&buf[..bytes_read]).await.unwrap();
    }
}
