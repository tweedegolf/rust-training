use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::string::String;
use std::sync::mpsc::SyncSender;

use crate::message::{KeepAlive, Measurement};

pub mod database;
pub mod message;

pub fn handle_client(socket: TcpStream, backend: SyncSender<Measurement>) -> anyhow::Result<()> {
    let mut buffered = BufReader::new(socket);

    loop {
        let mut line = String::new();
        buffered.read_line(&mut line)?;

        match serde_json::from_str(&line) {
            Ok(parsed) => backend.send(parsed)?,
            Err(e) => eprintln!("Could not deserialze {line:?}, because {e:?}"),
        }

        // Make sure the sensor knows everything is still ok...
        // Since the sensor sends a new measurement every second, and we only have to send a
        // KeepAlive every 5 seconds this is fine
        let msg = KeepAlive {
            everything_is_fine: true,
        };
        let mut json = serde_json::to_string(&msg)?;
        json.push('\n');
        buffered.get_ref().write_all(json.as_bytes())?;
    }
}
