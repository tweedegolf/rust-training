#![no_main]
#![no_std]

use embassy_nrf::{self as hal, twim::Twim};
use hal::twim;
use rtt_target::{rprintln, rtt_init_print};


hal::bind_interrupts!(struct Irqs {
    SPIM0_SPIS0_TWIM0_TWIS0_SPI0_TWI0 => twim::InterruptHandler<hal::peripherals::TWISPI0>;
});

#[cortex_m_rt::entry]
fn main() -> ! {
    // Init RTT control block
    rtt_init_print!();

    let _cp = cortex_m::Peripherals::take().unwrap();
    // Use ``dp` to get a handle to the peripherals
    let dp = hal::init(Default::default());

    rprintln!("Starting");

    let config = twim::Config::default();
    let mut twim0 = Twim::new(dp.TWISPI0, Irqs, dp.P0_03, dp.P0_04, config);

    rprintln!("Reading...");

    let mut buf = [0u8; 16];
    twim0.blocking_write_read(0xAB, &mut [0x00], &mut buf).unwrap();

    rprintln!("Read: {:02x?}", buf);
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
