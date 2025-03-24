#![no_main]
#![no_std]

use embassy_nrf as hal;
use hal::twim;
use rtt_target::{rprintln, rtt_init_print};

// STEP 1: Bind the interrupt of TWISPI0 to the TWIM ISR

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    // We prefix a variable name with an underscore to
    // turn off warnings about it not being used.
    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use `dp` to get a handle to the TWIM peripheral
    let dp = hal::init(Default::default());

    rprintln!("Starting");


    // STEP 2 Set up the TWISPI0 peripheral (that supports I2C), so that it uses
    // P0.16 for SDA and P0.08 for SCL
    
    // STEP 3 Read the ID register of the LSM303AGR's accelerometer

    // STEP 4 Print or assert the ID

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
