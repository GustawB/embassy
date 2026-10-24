#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_ti_cc2650::{
    bind_interrupts,
    chip::peripherals,
    debug_print::{DebugPrint, debug_print},
    gpio::{Level, Output},
    i2c::{self, I2C},
};
use embassy_time::Timer;
use panic_probe as _;

bind_interrupts!(struct Irqs {
    I2C => i2c::InterruptHandler<peripherals::I2C>;
});

const TMP431A_ADDR: u8 = 0x4C;
const TMP431B_ADDR: u8 = 0x4D;
const CR1_WRITE: u8 = 0x09;
const ONE_SHOT_REQUEST_WRITE_ADDR: u8 = 0x0F;
const TEMP_READ_ADDR: u8 = 0x00;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_ti_cc2650::init();
    let i2c = I2C::new(p.I2C, Irqs);
    let mut _l = Output::new(p.P_06, Level::High);
    let _debug_print = DebugPrint::new(p.P_28, p.P_29);

    // Set configuration register 1 write address.
    let _ = i2c.write_byte(TMP431A_ADDR, CR1_WRITE).await;
    // Set oneshot flag.
    let _ = i2c.write_byte(TMP431A_ADDR, (1 << 6) | (1 << 2)).await;

    let mut temp_buffer = [0u8; 3];
    loop {
        Timer::after_millis(1000).await;
        // Start one-shot request.
        let _ = i2c.write_byte(TMP431A_ADDR, ONE_SHOT_REQUEST_WRITE_ADDR).await;
        Timer::after_millis(100).await;
        // Set temperature high byte read address.
        let _ = i2c.write_byte(TMP431A_ADDR, TEMP_READ_ADDR).await;
        // Read temperature high byte.
        let mut tmp_byte = 0u8;
        let _ = i2c.read_byte(TMP431A_ADDR, &mut tmp_byte).await;

        for i in 0..3 {
            temp_buffer[i] = b'0';
        }
        let mut idx = 2;
        loop {
            let tmp = tmp_byte % 10;
            tmp_byte /= 10;
            temp_buffer[idx] = tmp + '0' as u8;
            idx -= 1;
            if tmp_byte == 0 {
                break;
            }
        }

        debug_print("Temperature: ".as_bytes());
        debug_print(&temp_buffer);
        debug_print("\n".as_bytes());
    }
}
