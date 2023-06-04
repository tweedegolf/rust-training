//! Hardware test application, used to verify that everything is connected
//! and works as it should.
#![no_std]
#![no_main]
use core::{fmt::Write};
use cortex_m_rt::entry;
use hal::{
    pac,
    uarte::{Baudrate, Parity, Pins},
};
use lis3dh::accelerometer::Accelerometer;

use workshop_examples::hal;

#[entry]
fn start() -> ! {
    // Get a handle to the nRF52840 device peripherals
    let peripherals = pac::Peripherals::take().unwrap();

    // Initialize port0
    let port0 = hal::gpio::p0::Parts::new(peripherals.P0);

    // Receiving pin, initialize as input
    let rxd = port0.p0_08.into_floating_input().degrade();

    // Transmitting pin, initialize as output
    let txd = port0
        .p0_06
        .into_push_pull_output(hal::gpio::Level::Low)
        .degrade(); // Erase the type, creating a generic pin

    // Create Pins struct to pass to Uarte
    let uart_pins = Pins {
        rxd,
        txd,
        // We don't use cts/rts
        cts: None, // Clear to send pin
        rts: None, // Request to send pin
    };

    // Initialize UART peripheral with standard configuration
    let mut uart = hal::Uarte::new(
        peripherals.UARTE0, // Take peripheral handle by value
        uart_pins,          // Take pins by value
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    defmt::println!("Printing message to virtual COM port...");
    write!(&mut uart, "Hello from Rust on the nRF52!\r\n").unwrap();
    defmt::println!("Done! Please check if you received it");

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

    defmt::println!(
        "Now going to initialize the LIS3DH driver. Please check connection if this goes wrong"
    );

    let mut lis3dh = lis3dh::Lis3dh::new_i2c(i2c, lis3dh::SlaveAddr::Default).unwrap();

    defmt::println!(
        "Found Lis3dh. Device id: {}",
        lis3dh.get_device_id().unwrap()
    );

    defmt::println!("Printing acc measurements to virtual COM port. Please check that this works. Press CTRL+C to quit");
    loop {
        let sample = lis3dh.accel_norm().unwrap();
        write!(
            &mut uart,
            "X: {:.3}\nY: {:.3}\nZ: {:.3}\r\n",
            sample.x, sample.y, sample.z
        )
        .unwrap();
    }
}
