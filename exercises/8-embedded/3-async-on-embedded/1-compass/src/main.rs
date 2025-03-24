#![no_main]
#![no_std]

use dial::{dir_channel, Dial, Direction};
use embassy_nrf::{self as hal, peripherals::TWISPI0, twim::Twim};
use embedded_hal_async::delay::DelayNs;
use hal::twim;
use lsm303agr::{interface::I2cInterface, mode::MagOneShot, Lsm303agr, MagnetometerId};
use rtt_target::{rprintln, rtt_init_print};

use panic_rtt_target as _; // Panic handler

mod dial;

hal::bind_interrupts!(struct Irqs {
    TWISPI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
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
    let delay = embassy_time::Delay;

    let dial: Dial = todo!("Initialize Dial");

    let mut sensor: Lsm303agr<I2cInterface<Twim<TWISPI0>>, MagOneShot> =
        todo!("Initialize LSM303AGR driver given the twim0 peripheral");
    let id: MagnetometerId = todo!("Read the magnetometer ID using the driver");
    rprintln!("{:#02x?}", id);

    todo!("Initialize the driver");
    todo!("Set magnetometer mode to high resolution and output data rate to 100Hz");

    todo!("Change the magnetometer to continuous mode");
    todo!("Enable magnetometer offset cancellation");

    loop {
        todo!("Read data and update the dial accordingly");

        // Steps:
        // - Read the MagneticField data
        // - Convert to a direction using: Direction::from(mag_data)
        // - Update the dial: dial.set_light_direction(direction)
    }
}
