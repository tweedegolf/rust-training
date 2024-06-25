#![no_main]
#![no_std]

use dial::{dir_channel, run_dial, Dial, Direction};
use embassy_nrf::{self as hal, twim::Twim};
use embassy_time::Delay;
use embedded_hal_async::delay::DelayNs;
use hal::twim;
use lsm303agr::Lsm303agr;
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _; // Panic handler

mod dial;

hal::bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
});

#[embassy_executor::main]
async fn main(s: embassy_executor::Spawner) -> ! {

    // Init RTT control block
    rtt_init_print!();

    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use ``dp` to get a handle to the peripherals
    let dp = hal::init(Default::default());

    rprintln!("Starting");

    let config = twim::Config::default();
    let twim0 = Twim::new(dp.TWISPI0, Irqs, dp.P0_16, dp.P0_08, config);

    let dial = Dial::new(
        dp.P0_21, dp.P0_22, dp.P0_15, dp.P0_24, dp.P0_19, dp.P0_28, dp.P0_11, dp.P0_31, dp.P1_05,
        dp.P0_30,
    );

    // dial.light_only(Direction::North);
    let (rx, tx) = dir_channel();
    s.spawn(run_dial(dial, rx)).unwrap();

    let mut sensor = Lsm303agr::new_with_i2c(twim0);
    let id = sensor.magnetometer_id().await.unwrap();
    rprintln!("{:#02x?}", id);

    sensor.init().await.unwrap();
    sensor
        .set_mag_mode_and_odr(
            &mut Delay,
            lsm303agr::MagMode::HighResolution,
            lsm303agr::MagOutputDataRate::Hz100,
        )
        .await
        .unwrap();
    let Ok(mut sensor) = sensor.into_mag_continuous().await else {
        panic!()
    };
    
    sensor.enable_mag_offset_cancellation().await.unwrap();
    loop {
        if sensor.mag_status().await.unwrap().xyz_new_data() {
            let data = sensor.magnetic_field().await.unwrap();
            let dir = Direction::from(data);
            rprintln!(
                "Magnetic field: x {} y {} z {}; Dir: {:?}",
                data.x_unscaled(),
                data.y_unscaled(),
                data.z_unscaled(),
                dir
            );
            tx.send(dir).await;
            // dial.light_only(dir);
        } else {
            // dial.light_only(Direction::None);
            tx.send(Direction::None).await;
            rprintln!("No data")
        }
        Delay.delay_ms(10).await;
    }
}
