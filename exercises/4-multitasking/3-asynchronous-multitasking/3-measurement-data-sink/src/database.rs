use std::collections::BTreeMap;
use std::sync::mpsc::Receiver;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::message::Measurement;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct DatabaseRow {
    seconds_since_unix_epoch: u64,
    room_id: usize,
    avg_temperature: f64,
    avg_humidity: f64,
    total_fires: u64,
}

pub fn run(receiver: Receiver<Measurement>) -> anyhow::Result<()> {
    let mut database = csv::Writer::from_path("database.csv")?;

    loop {
        // Accumulate data for one minute
        std::thread::sleep(Duration::from_secs(60));

        // Now accumulate data from the last minute
        let now = SystemTime::now();

        // Group measurements by room
        let mut rooms: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for meas in receiver.try_iter() {
            rooms.entry(meas.room_id).or_default().push(meas);
        }

        println!("Got data for {} rooms", rooms.len());

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
