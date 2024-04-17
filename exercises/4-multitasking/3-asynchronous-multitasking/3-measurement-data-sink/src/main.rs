use std::net::TcpListener;
use std::sync::mpsc::sync_channel;

use data_sink::handle_client;

fn main() -> anyhow::Result<()> {
    let (sender, receiver) = sync_channel(1_000_000);
    std::thread::spawn(move || data_sink::database::run(receiver));

    let listener = TcpListener::bind("[::1]:8080")?;

    loop {
        let (socket, _) = listener.accept()?;
        let sender = sender.clone();
        std::thread::spawn(move || handle_client(socket, sender));
    }
}
