#![no_main]
#![no_std]

use embassy_nrf as hal;
use rtt_target::{rprintln, rtt_init_print};

mod lsm303agr;

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    // We prefix a variable name with an underscore to
    // turn off warnings about it not being used.
    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use `dp` to get a handle to the TWISPI0 peripheral
    let dp = hal::init(Default::default());

    rprintln!("Starting");

    // Initialize your driver, read out the acceleration values and print them

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
