#![no_main]
#![no_std]

use rtt_target::{rprintln, rtt_init_print};
use nrf52840_hal as hal;

mod lis3dh;

#[cortex_m_rt::entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let _dp = hal::pac::Peripherals::take().unwrap();

    rtt_init_print!(BlockIfFull);
    rprintln!("Starting");

    // Set up the LIS3DH pins

    // Set up the i2c

    // Read the ID register of the LIS3DH

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
        cortex_m::asm::bkpt();
    }
}
