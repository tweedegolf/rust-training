#![no_main]
#![no_std]

use nrf52833_hal as hal;
use rtt_target::{rprintln, rtt_init_print};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    // We prefix a variable name with an underscore to
    // turn off warnings about it not being used.
    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use `dp` to get a handle to the TWIM peripheral
    let dp = hal::pac::Peripherals::take().unwrap();

    rprintln!("Starting");

    // Set up the I2C pins

    // Set up the TWIM peripheral (that supports I2C)

    // Read the ID register of the LSM303AGR's accelerometer

    // Print or assert the ID

    exit();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    rprintln!("{}", info);
    exit();
}

fn exit() -> ! {
    loop {
        rprintln!("Exiting now");
        cortex_m::asm::bkpt();
    }
}
