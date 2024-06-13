//! Hardware test application, used to verify that everything is connected
//! and works as it should.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use hal::pac;
use lis3dh::accelerometer::Accelerometer;

use workshop_examples::hal;

#[entry]
fn start() -> ! {
    rtt_target::rtt_init_print!();
    // Get a handle to the nRF52840 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();

    // Initialize port0
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    // Set up lis3dh driver over I2C
    let twim0_scl = port0.p0_27.into_floating_input().degrade();
    let twim0_sda = port0.p0_26.into_floating_input().degrade();
    let i2c = hal::twim::Twim::new(
        peripherals.TWIM0,
        hal::twim::Pins {
            scl: twim0_scl,
            sda: twim0_sda,
        },
        hal::twim::Frequency::K400,
    );

    rtt_target::rprintln!(
        "Now going to initialize the LIS3DH driver. Please check connection if this goes wrong"
    );

    let addr = if cfg!(feature = "alternate-addr") {
        lis3dh::SlaveAddr::Alternate
    } else {
        lis3dh::SlaveAddr::Default
    };

    let mut lis3dh = lis3dh::Lis3dh::new_i2c(i2c, addr).unwrap();

    rtt_target::rprintln!(
        "Found Lis3dh. Device id: {}",
        lis3dh.get_device_id().unwrap()
    );

    rtt_target::rprintln!("Here's a couple of accelerometer samples:");
    loop {
        let sample = lis3dh.accel_norm().unwrap();
        rtt_target::rprintln!("X: {:.3}\tY: {:.3}\tZ: {:.3}", sample.x, sample.y, sample.z);
    }
}
