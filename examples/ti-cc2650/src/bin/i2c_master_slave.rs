#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::{
    bind_interrupts,
    chip::peripherals,
    debug_print::{DebugPrint, debug_print},
    i2c::Instance,
    i2c::{self, I2CMaster, I2CSlave},
};
use embassy_time::Timer;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C => i2c::InterruptHandler<peripherals::I2C>;
});

fn read_callback(data: u8) {
    debug_print(&[data]);
}

#[embassy_executor::task]
async fn slave_task(slave: I2CSlave<T: Instance>) -> ! {
    loop {
        slave.run(read_callback, || return 0).await;
    }
}

const SLAVE_ADDR: u8 = 0x69;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let i2c_master = I2CMaster::new(p.I2C, Irqs);
    let i2c_slave = I2CSlave::new(p.I2C, SLAVE_ADDR, Irqs);
    let _debug_print = DebugPrint::new(p.P_28, p.P_29);

    let hello = "Hello World!\n".as_bytes();
    loop {
        Timer::after_secs(1).await;
        let _ = i2c_master.write(SLAVE_ADDR, hello).await;
    }
}
