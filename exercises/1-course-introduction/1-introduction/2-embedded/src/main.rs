//! Hardware test application, used to verify that everything is connected
//! and works as it should.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use hal::pac;

use lsm303agr::{AccelMode, AccelOutputDataRate};
use rtt_target::rprintln;
use workshop_examples::hal;

#[entry]
fn start() -> ! {
    // Initialize the RTT control block
    rtt_target::rtt_init_print!();
    // Get a handle to the nRF52833 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();
    // Get a handle to the Cortex-M4 core peripherals
    let core_periperals = cortex_m::Peripherals::take().unwrap();

    // Initialize port0
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    // Set up TWIM peripheral using the board's internal I2C lines
    let twim0_scl = port0.p0_08.into_floating_input().degrade();
    let twim0_sda = port0.p0_16.into_floating_input().degrade();
    let i2c = hal::twim::Twim::new(
        peripherals.TWIM0,
        hal::twim::Pins {
            scl: twim0_scl,
            sda: twim0_sda,
        },
        hal::twim::Frequency::K100,
    );

    // Setup SysTick-based delay
    let mut delay = nrf52833_hal::Delay::new(core_periperals.SYST);


    rprintln!(
        "Now going to initialize the LSM303AGR driver. Please check your board if this goes wrong"
    );

    let mut motion = lsm303agr::Lsm303agr::new_with_i2c(i2c);

    let id = motion.accelerometer_id().expect("Error getting accelerometer ID");
    if !id.is_correct() {
        panic!("Accelerometer had unexpected ID {:#x}", id.raw())
    }
    rprintln!("Found LS303AGR. Accelerometer device id: {:#x}", id.raw());


    motion.init().expect("Error initializing motion sensor");

    motion
        .set_accel_mode_and_odr(
            &mut delay,
            AccelMode::HighResolution,
            AccelOutputDataRate::Hz400,
        )
        .unwrap();


    rprintln!("Here's a couple of accelerometer samples:");
    loop {
        let (x, y, z) = motion.acceleration().unwrap().xyz_mg();
        rprintln!("X: {:03.3}mg\tY: {:03.3}mg\tZ: {:03.3}mg", x, y, z);
    }
}
