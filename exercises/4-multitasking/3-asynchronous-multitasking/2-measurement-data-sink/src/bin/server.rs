// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::{TcpListener, TcpStream};
// use tokio::sync::mpsc;
use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Context;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use data_sink::{KeepAlive, Measurement};

fn main() -> anyhow::Result<()> {
    // Setup logging
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env()
        .context("Invalid log filter in RUST_LOG")?;
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Setup backend
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || database(receiver)); // TODO: Change to tokio::task

    let listener = TcpListener::bind("[::1]:8080")?;

    loop {
        let (socket, _) = listener.accept()?;
        let sender = sender.clone();
        std::thread::spawn(move || handle_client(socket, sender)); // TODO: Change to tokio::task
    }
}

// TODO: make this an async fn
pub fn handle_client(socket: TcpStream, backend: mpsc::Sender<Measurement>) -> anyhow::Result<()> {
    tracing::info!("New connection");

    let mut buffered = BufReader::new(socket);

    loop {
        let mut line = String::new();

        buffered.read_line(&mut line)?;

        // Make sure the sensor knows everything is still ok...
        // Since the sensor sends a new measurement every second, and we only have to send a
        // KeepAlive every 5 seconds this is fine
        let msg = KeepAlive {
            everything_is_fine: true,
        };
        let mut json = serde_json::to_string(&msg)?;
        json.push('\n');
        buffered.get_mut().write_all(json.as_bytes())?;

        match serde_json::from_str::<Measurement>(&line) {
            Ok(parsed) => {
                tracing::debug!(node_id = parsed.node_id, "Received measurement");
                backend.send(parsed)?;
            }
            Err(err) => tracing::error!(?err, line, "Failed to deserialize"),
        }
    }
}

// TODO: make this an async fn
#[tracing::instrument(skip_all)]
pub fn database(receiver: Receiver<Measurement>) -> anyhow::Result<()> {
    let mut database = csv::Writer::from_path("database.csv")?;

    loop {
        // Accumulate data for one minute
        std::thread::sleep(Duration::from_secs(60));

        // Now accumulate data from the last minute
        let now = SystemTime::now();

        // Group measurements by room
        let mut rooms: BTreeMap<_, Vec<_>> = BTreeMap::new();
        while let Ok(meas) = receiver.try_recv() {
            rooms.entry(meas.room_id).or_default().push(meas);
        }

        // NOTE don't bother about the code below, unless you have extra time

        let measurement_count: usize = rooms.values().map(Vec::len).sum();
        tracing::info!(
            rooms = rooms.len(),
            measurement_count,
            "Collected data batch"
        );

        // Average by room and append to highly sophisticated database
        for (room, measurements) in rooms {
            let mut row = DatabaseRow {
                seconds_since_unix_epoch: now.duration_since(UNIX_EPOCH).unwrap().as_secs(),
                room_id: room,
                avg_temperature: 0.0,
                avg_humidity: 0.0,
                total_fires: 0,
            };
            let cnt = measurements.len();

            for meas in measurements {
                assert_eq!(room, meas.room_id);
                row.avg_temperature += meas.temperature;
                row.avg_humidity += meas.rel_humidity;
                row.total_fires += meas.fires;
            }
            row.avg_temperature /= cnt as f64;
            row.avg_humidity /= cnt as f64;

            database.serialize(&row)?;
        }

        database.flush()?;
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct DatabaseRow {
    seconds_since_unix_epoch: u64,
    room_id: usize,
    avg_temperature: f64,
    avg_humidity: f64,
    total_fires: u64,
}
