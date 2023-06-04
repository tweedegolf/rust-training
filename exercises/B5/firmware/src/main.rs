#![no_std]
#![no_main]

use firmware::hal;

use firmware::uarte::{Baudrate, Parity, Pins as UartePins, TimeoutUarte};

#[allow(unused_imports)]
use hal::prelude::*;

use embedded_hal::timer::CountDown;
use format::{DeviceToServer, ServerToDevice};
use hal::{
    gpio::{p0, Level},
    pac::{TIMER0, UARTE0},
    ppi::{self, Ppi0},
    Timer,
};
use postcard::accumulator::{CobsAccumulator, FeedResult};
use systick_monotonic::*;

use rtt_target::{rprintln, rtt_init_print};

#[rtic::app(
    device=firmware::hal::pac,
    peripherals=true,
    dispatchers = [SWI0_EGU0, SWI1_EGU1, SWI2_EGU2],
)]
mod app {
    use super::*;

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<1000>; // 1000 Hz / 1 ms granularity

    #[local]
    struct LocalResources {
        accumulator: CobsAccumulator<32>,
    }

    // The resources that are to be shared between tasks
    #[shared]
    struct SharedResources {
        uarte0: TimeoutUarte<UARTE0, TIMER0, Ppi0>,
    }

    // Initialize peripherals, before interrupts are unmasked
    // Initializes and returns all resources that need to be dynamically instantiated
    #[init]
    fn init(ctx: init::Context) -> (SharedResources, LocalResources, init::Monotonics) {
        rtt_init_print!(BlockIfFull);
        rprintln!("Starting");

        // Enable systick counter for task scheduling
        let mono = Systick::new(ctx.core.SYST, 64_000_000);

        // Initialize UARTE0
        // Initialize port0
        let port0 = p0::Parts::new(ctx.device.P0);
        // Initialize PPI
        let ppi = ppi::Parts::new(ctx.device.PPI);

        // UART Receiving pin, initialize as input
        let rxd = port0.p0_08.into_floating_input().degrade();

        // UART Transmitting pin, initialize as output
        let txd = port0.p0_06.into_push_pull_output(Level::Low).degrade(); // Erase the type, creating a generic pin

        // Create Pins struct to pass to Uarte
        let uarte_pins = UartePins {
            rxd,
            txd,
            // We don't use cts/rts
            cts: None, // Clear to send pin
            rts: None, // Request to send pin
        };

        // A timer that is used to time-out UARTE0 read transactions,
        // so the device can react to commands even if the
        // UARTE0 RX FIFO is not yet full
        let mut timer0 = Timer::periodic(ctx.device.TIMER0);
        timer0.start(200_000u32); // 200 ms period

        // Initialize UARTE0 peripheral with standard configuration
        let uarte0 = TimeoutUarte::init(
            ctx.device.UARTE0, // Take peripheral handle by value
            uarte_pins,        // Take pins by value
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
            timer0,   // Take TIMER0 by value
            ppi.ppi0, // Take PPI channel 0 by value
        );

        // An accumulator for postcard-COBS messages
        let accumulator = CobsAccumulator::new();

        (
            SharedResources { uarte0 },
            LocalResources { accumulator },
            init::Monotonics(mono),
        )
    }

    // Defines what happens when there's nothing left to do
    #[idle]
    fn idle(_ctx: idle::Context) -> ! {
        loop {
            // Go to sleep, waiting for an interrupt
            cortex_m::asm::wfi();
        }
    }

    // Do something with a message that just came in
    #[task(capacity = 5, priority = 5)]
    fn handle_message(_ctx: handle_message::Context, msg: ServerToDevice) {
        rprintln!("Received message: {:?}. What do I need to do now?", msg);
        let ServerToDevice {
            say_hello,
            set_led_status,
            ..
        } = msg;

        if say_hello {
            send_message::spawn(DeviceToServer {
                said_hello: true,
                ..DeviceToServer::default()
            })
            .ok();
        }

        if let Some((led_id, enabled)) = set_led_status {
            // TODO react to an incoming message, possibly by spawning a newly defined task.
            // This task should set the led status as specified by the command.
        }
    }

    // Send a message over UARTE0
    // This task waits until the last TX transaction is completed
    #[task(capacity = 10, shared = [uarte0], priority = 1)]
    fn send_message(mut ctx: send_message::Context, msg: DeviceToServer) {
        rprintln!("Sending message: {:?}", &msg);
        let mut buf = [0; 32];
        // Serialize message
        if let Ok(bytes) = postcard::to_slice_cobs(&msg, &mut buf) {
            // Repeatedly try to write the message. We need to lock uarte0 here,
            // as this task might be pre-empted by another task that uses uarte0.
            // uarte0.try_start_tx returns an Err variant if there is already a
            // TX transaction going on.
            while let Err(_) = ctx.shared.uarte0.lock(|uarte0| uarte0.try_start_tx(&bytes)) {
                // Go to sleep to avoid busy waiting
                cortex_m::asm::wfi();
            }
        } else {
            rprintln!(
                "Could not serialize message {:?}. Please increase buffer size.",
                msg
            )
        }
    }

    // React to an interrupt from UARTE0
    #[task(
        binds = UARTE0_UART0,
        priority = 6,
        shared = [uarte0],
    )]
    fn on_uarte0(mut ctx: on_uarte0::Context) {
        use firmware::uarte::UarteEvent::*;

        ctx.shared
            .uarte0
            // We need to lock here, as this task might be pre-empted by
            // higher-priority tasks that use uarte0.
            .lock(|uarte0| match uarte0.get_clear_event() {
                Some(EndRx) => {
                    // Read transaction ended, spawn read task
                    read_uarte0::spawn().ok();
                }
                Some(EndTx) => {
                    // This event causes the running
                    // send_message task to try sending once more.
                    // No need to handle it here.
                }
                _ => (),
            });
    }

    // Software task that reads the UARTE0 DMA buffer,
    // deserializes the data, and spawns the `handle_message`
    // task.
    #[task(
        priority = 7,
        shared = [uarte0],
        local = [accumulator],
    )]
    fn read_uarte0(mut ctx: read_uarte0::Context) {
        let chunk = ctx.shared.uarte0.lock(|uarte0| uarte0.get_rx_chunk());
        match ctx.local.accumulator.feed(chunk) {
            FeedResult::Consumed => {}
            FeedResult::OverFull(_) => rprintln!("Accumulator full, dropping contents"),
            FeedResult::DeserError(_) => rprintln!("Deserialize error, throwing away message"),
            FeedResult::Success { data, .. } => handle_message::spawn(data)
                .expect("Could not start handle_message task, please increase its capacity."),
        }
    }
}
