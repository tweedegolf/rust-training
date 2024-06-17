#![no_std]

use panic_probe as _;

pub use nrf52833_hal as hal;

/// Terminates the application and makes `probe-rs run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
