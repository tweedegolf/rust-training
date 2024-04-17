use std::net::IpAddr;

use anyhow::{anyhow, Context};
use clap::{arg, Parser};
use rand::{Rng, thread_rng};
use rand::distributions::{Bernoulli, Open01, Uniform};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::select;
use tokio::signal::ctrl_c;
use tokio::task::JoinSet;
use tokio::time::{Duration, Instant, interval, Interval, timeout_at};
use tokio::time::error::Elapsed;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

use many_sensors::{KeepAlive, Measurement, Mood};

/// A tool that simulates (potentially many) clients sending measurements
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Address of the server to connect to
    #[arg(default_value = "::1")]
    target: IpAddr,

    /// Port on the server to connect to
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Number of clients to spawn
    #[arg(short = 'n', long, default_value_t = 10)]
    clients: usize,

    /// Time between measurements
    #[arg(short, long, default_value = "1s")]
    interval: humantime::Duration,
}

#[tokio::main]
#[tracing::instrument]
async fn main() -> anyhow::Result<()> {
    // Set up logging
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .context("Invalid log filter in RUST_LOG")?;
    tracing_subscriber::fmt().with_env_filter(filter).init();

    // Parse CLI args and make them live forever
    let args = Args::parse();
    let args = &*Box::leak(Box::new(args));
    tracing::info!(?args, "Going to start clients");

    // Start one task per simulated client
    let mut handles = JoinSet::new();
    for id in 0..args.clients {
        if id == 0 {
            handles.spawn(run_broken_client(id, args));
        } else {
            handles.spawn(run_client(id, args));
        }
    }
    tracing::info!("All clients started. Waiting for them to finish");

    // Wait for all tasks to finish, logging any errors
    let mut failures = 0;
    while let Some(result) = handles.join_next().await {
        match result {
            Ok(Ok(())) => {
                tracing::debug!("Client finished successfully")
            }
            Ok(Err(_)) => {
                tracing::debug!("Client finished with error");
                failures += 1;
            }
            Err(e) => {
                let panicked = e.is_panic();
                tracing::error!(?e, panicked, "Client died");
                failures += 1;
            }
        }
    }
    tracing::info!(failures, "All clients finished");

    if failures != 0 {
        Err(anyhow!("{failures} tasks ended with errors"))
    } else {
        Ok(())
    }
}

#[tracing::instrument(skip(args), err)]
async fn run_client(id: usize, args: &'static Args) -> anyhow::Result<()> {
    let mut sock = tokio::net::TcpStream::connect((args.target, args.port)).await?;
    let (rx, mut tx) = sock.split();
    let mut lines = BufReader::new(rx).lines();

    tracing::debug!("Connected to server");

    let mut last_keep_alive = Instant::now();
    let mut interval = interval(args.interval.into());
    loop {
        tokio::select! {
            res = handle_keep_alive(&mut lines, &mut last_keep_alive) => {
                res?;
            }
            res = send_measurement(&mut tx, &mut interval, id) => {
                res?;
            }
            res = ctrl_c() => {
                res.context("Failed to listen for Ctrl-C")?;

                tracing::debug!("Shutting down after Ctrl-C");
                return Ok(());
            }
        }
    }
}

#[tracing::instrument(skip(args), err)]
async fn run_broken_client(id: usize, args: &'static Args) -> anyhow::Result<()> {
    let mut sock = tokio::net::TcpStream::connect((args.target, args.port)).await?;
    let (rx, mut tx) = sock.split();
    let mut lines = BufReader::new(rx).lines();

    tracing::debug!("Connected to server");

    let mut interval = interval(Duration::from_millis(1000));
    loop {
        let measurement = measure(id);
        let mut json =
            serde_json::to_string(&measurement).context("Could not serialize measurement")?;
        json.push('\n');

        for bytes in json.as_bytes().chunks(10) {
            tx.write_all(bytes)
                .await
                .context("Could not send to server")?;
            tx.flush().await?;

            tokio::select! {
                _ = interval.tick() => {}
                res = ctrl_c() => {
                    res.context("Failed to listen for Ctrl-C")?;
                    tracing::debug!("Shutting down after Ctrl-C");
                    return Ok(());
                }
            }
        }

        tracing::debug!(
            "Sent measurement... only took {} seconds",
            json.as_bytes().len() / 10
        );
    }
}

async fn send_measurement(
    tx: &'_ mut WriteHalf<'_>,
    interval: &'_ mut Interval,
    id: usize,
) -> anyhow::Result<()> {
    interval.tick().await;
    let measurement = measure(id);
    let mut json =
        serde_json::to_string(&measurement).context("Could not serialize measurement")?;
    json.push('\n');
    tx.write_all(json.as_bytes())
        .await
        .context("Could not send to server")?;
    tracing::debug!(?measurement, "Sent measurement");
    Ok(())
}

async fn handle_keep_alive(
    lines: &'_ mut Lines<BufReader<ReadHalf<'_>>>,
    last_keep_alive: &'_ mut Instant,
) -> anyhow::Result<()> {
    let deadline = *last_keep_alive + Duration::from_secs(5);
    let line = match timeout_at(deadline, lines.next_line()).await {
        Ok(Ok(Some(line))) => line,
        Ok(Err(_)) | Ok(Ok(None)) => {
            anyhow::bail!("Lost connection to the server");
        }
        Err(Elapsed { .. }) => {
            anyhow::bail!("Did not receive a keep-alive in the last 5 seconds! Shutting down...");
        }
    };

    let msg = serde_json::from_str(&line);
    match msg {
        Ok(KeepAlive {
            everything_is_fine: true,
        }) => {
            tracing::debug!("Server said everything is fine...");
            *last_keep_alive = Instant::now()
        }
        Ok(KeepAlive {
            everything_is_fine: false,
        }) => {
            tracing::warn!("Server reported there is something wrong!");
        }
        Err(e) => {
            tracing::error!(line, %e, "Server send weird message, ignoring...");
        }
    }

    Ok(())
}

fn measure(id: usize) -> Measurement {
    let mut rng = thread_rng();

    Measurement {
        node_id: id,
        room_id: id % 256,
        temperature: rng.sample(Uniform::new(15.0, 25.0)),
        rel_humidity: rng.sample(Open01),
        fires: if rng.sample(Bernoulli::new(0.01).unwrap()) {
            1
        } else {
            0
        },
        mood: Mood::Good,
    }
}
