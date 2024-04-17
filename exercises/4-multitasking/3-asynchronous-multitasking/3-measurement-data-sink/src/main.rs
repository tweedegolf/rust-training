use std::time::Duration;

use anyhow::Context;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::interval;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

use data_sink::message::{KeepAlive, Measurement};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .context("Invalid log filter in RUST_LOG")?;
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Setup backend
    let (sender, receiver) = mpsc::channel(1024);
    tokio::task::spawn(data_sink::database::run(receiver));

    let listener = TcpListener::bind("[::1]:8080").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let sender = sender.clone();
        tokio::spawn(handle_client(socket, sender));
    }
}

// NOTE: make this an async fn
#[tracing::instrument(skip_all, fields(peer_addr = %socket.peer_addr()?), err)]
pub async fn handle_client(
    socket: TcpStream,
    backend: mpsc::Sender<Measurement>,
) -> anyhow::Result<()> {
    tracing::info!("New connection");

    let mut buffered = BufReader::new(socket);

    let mut interval = interval(Duration::from_secs(4));

    loop {
        let mut line = String::new();
        tokio::select! {
            res =  buffered.read_line(&mut line) => {
                res?;
                match serde_json::from_str::<Measurement>(&line) {
                    Ok(parsed) => {
                        tracing::debug!(node_id = parsed.node_id, "Received measurement");
                        backend.send(parsed).await?;
                    }
                    Err(err) => tracing::error!(?err, line, "Failed to deserialize"),
                }
            }
            _ = interval.tick() => {
                let msg = KeepAlive {
                    everything_is_fine: true,
                };
                let mut json = serde_json::to_string(&msg)?;
                json.push('\n');
                buffered.get_mut().write_all(json.as_bytes()).await?;
            }
        }
    }
}
