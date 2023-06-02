#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_nrf::gpio::{AnyPin, Level, Output, OutputDrive, Pin};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

#[derive(defmt::Format)]
enum LedState {
    On,
    Off,
}

static CHANNEL: StaticCell<Channel<NoopRawMutex, LedState, 1>> = StaticCell::new();

#[embassy_executor::task]
async fn send_task(sender: Sender<'static, NoopRawMutex, LedState, 1>) {
    defmt::debug!("Started send_task");

    loop {
        sender.send(LedState::On).await;
        defmt::trace!("Sent LED on");
        Timer::after(Duration::from_millis(1000)).await;

        sender.send(LedState::Off).await;
        defmt::trace!("Sent LED off");
        Timer::after(Duration::from_millis(1000)).await;
    }
}

#[embassy_executor::task]
async fn recv_task(led: AnyPin, receiver: Receiver<'static, NoopRawMutex, LedState, 1>) {
    defmt::debug!("Started recv_task");
    let mut led = Output::new(led, Level::Low, OutputDrive::Standard);

    loop {
        let received_state = receiver.recv().await;

        defmt::trace!("Received: {}", received_state);

        match received_state {
            LedState::On => led.set_high(),
            LedState::Off => led.set_low(),
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    defmt::info!("Starting up");
    let p = embassy_nrf::init(Default::default());

    defmt::info!("Initializing channel");
    let channel = CHANNEL.init(Channel::new());

    defmt::info!("Starting tasks");
    unwrap!(spawner.spawn(send_task(channel.sender())));
    unwrap!(spawner.spawn(recv_task(p.P0_13.degrade(), channel.receiver())));
}
