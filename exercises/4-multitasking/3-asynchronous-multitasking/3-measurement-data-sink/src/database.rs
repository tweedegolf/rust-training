use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::mpsc::Receiver;

use crate::message::Measurement;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct DatabaseRow {
    seconds_since_unix_epoch: u64,
    room_id: usize,
    avg_temperature: f64,
    avg_humidity: f64,
    total_fires: u64,
}

// TODO asyncify
#[tracing::instrument(skip_all)]
pub async fn run(mut receiver: Receiver<Measurement>) -> anyhow::Result<()> {
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

        let measurements: usize = rooms.values().map(Vec::len).sum();
        tracing::info!(rooms = rooms.len(), measurements, "Collected data batch");

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
