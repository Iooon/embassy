//! This example writes a burst larger than the 8 byte FIFO: all 16 channels of
//! a (pre-configured, auto-increment enabled) PCA9685 PWM controller in one go.
//!
//! Uses the LP-MSPM0L1306 board.

#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_mspm0::i2c::{Config, I2c};
use embassy_time::Timer;
use panic_halt as _;

const ADDRESS: u8 = 0x40;
/// First of the four registers (ON_L, ON_H, OFF_L, OFF_H) of channel 0.
const LED0_ON_L: u8 = 0x06;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_mspm0::init(Default::default());

    let instance = p.I2C0;
    let scl = p.PA1;
    let sda = p.PA0;

    let mut i2c = unwrap!(I2c::new_blocking(instance, scl, sda, Config::default()));

    let mut duty: u16 = 0;

    loop {
        // Register pointer plus 16 channels of ON = 0, OFF = duty.
        let mut to_write = [0u8; 1 + 16 * 4];
        to_write[0] = LED0_ON_L;
        for ch in 0..16 {
            to_write[3 + ch * 4..5 + ch * 4].copy_from_slice(&duty.to_le_bytes());
        }

        match i2c.blocking_write(ADDRESS, &to_write) {
            Ok(()) => info!("Wrote {} bytes, duty {}", to_write.len(), duty),
            Err(e) => error!("I2c Error: {:?}", e),
        }

        duty = (duty + 256) % 4096;
        Timer::after_millis(500).await;
    }
}
