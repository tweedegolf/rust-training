#![no_main]
#![no_std]

use rtt_target::{rprintln, rtt_init_print, ChannelMode};
use nrf52840_hal as hal;

mod lis3dh;


#[cortex_m_rt::entry]
fn main() -> ! {
    // We prefix a variable name with an underscore to
    // turn off warnings about it not being used.
    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use `dp` to get a handle to the TWIM peripheral
    let dp = hal::pac::Peripherals::take().unwrap();

    rtt_init_print!(ChannelMode::BlockIfFull);
    rprintln!("Starting");

    // Initialize your driver and read out the ID register with it

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
