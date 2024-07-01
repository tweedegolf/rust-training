#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    rtt_init_print!();
    let p = embassy_nrf::init(Default::default());
    let mut row_3 = Output::new(p.P0_15, Level::Low, OutputDrive::Standard);
    let _col_3 = Output::new(p.P0_31, Level::Low, OutputDrive::Standard);

    loop {
        row_3.set_high();
        rprintln!("🌞");
        Timer::after_millis(500).await;
        row_3.set_low();
        rprintln!("🌚");
        Timer::after_millis(500).await;
    }
}
