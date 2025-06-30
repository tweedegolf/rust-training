#![allow(dead_code)]

use embassy_futures::select::{select, Either};
use embassy_nrf::gpio::{self, Pin};
use embassy_sync::{blocking_mutex::raw::NoopRawMutex, once_lock::OnceLock};
use lsm303agr::MagneticField;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthNorthEast,
    NorthEast,
    EastNorthEast,
    East,
    EastSouthEast,
    SouthEast,
    SouthSouthEast,
    South,
    SouthSouthWest,
    SouthWest,
    WestSouthWest,
    West,
    WestNorthWest,
    NorthWest,
    NorthNorthWest,
    None,
}

impl Direction {
    fn led_index(self) -> (usize, usize) {
        match self {
            Self::North => (0, 2),
            Self::NorthNorthEast => (0, 3),
            Self::NorthEast => (0, 4),
            Self::EastNorthEast => (1, 4),
            Self::East => (2, 4),
            Self::EastSouthEast => (3, 4),
            Self::SouthEast => (4, 4),
            Self::SouthSouthEast => (4, 3),
            Self::South => (4, 2),
            Self::SouthSouthWest => (4, 1),
            Self::SouthWest => (4, 0),
            Self::WestSouthWest => (3, 0),
            Self::West => (2, 0),
            Self::WestNorthWest => (1, 0),
            Self::NorthWest => (0, 0),
            Self::NorthNorthWest => (0, 1),
            Self::None => (2, 2),
        }
    }

    fn from_xy(x: f32, y: f32) -> Self {
        let alpha = libm::atan2f(-y, x);
        let part =
            libm::roundf((alpha + core::f32::consts::PI) * 8.0 / core::f32::consts::PI) as u32;

        match (part + 4) % 16 {
            0 => Self::North,
            1 => Self::NorthNorthEast,
            2 => Self::NorthEast,
            3 => Self::EastNorthEast,
            4 => Self::East,
            5 => Self::EastSouthEast,
            6 => Self::SouthEast,
            7 => Self::SouthSouthEast,
            8 => Self::South,
            9 => Self::SouthSouthWest,
            10 => Self::SouthWest,
            11 => Self::WestSouthWest,
            12 => Self::West,
            13 => Self::WestNorthWest,
            14 => Self::NorthWest,
            15 => Self::NorthNorthWest,
            x => unreachable!("{x}"),
        }
    }
}

impl From<MagneticField> for Direction {
    fn from(field: MagneticField) -> Self {
        let (x, y, _) = field.xyz_unscaled();
        let x = x as f32;
        let y = y as f32;

        Self::from_xy(x, y)
    }
}
type Row1Pin = embassy_nrf::peripherals::P0_21;
type Row2Pin = embassy_nrf::peripherals::P0_22;
type Row3Pin = embassy_nrf::peripherals::P0_15;
type Row4Pin = embassy_nrf::peripherals::P0_24;
type Row5Pin = embassy_nrf::peripherals::P0_19;

type Col1Pin = embassy_nrf::peripherals::P0_28;
type Col2Pin = embassy_nrf::peripherals::P0_11;
type Col3Pin = embassy_nrf::peripherals::P0_31;
type Col4Pin = embassy_nrf::peripherals::P1_05;
type Col5Pin = embassy_nrf::peripherals::P0_30;

pub struct Dial {
    rows: [gpio::Output<'static>; 5],
    cols: [gpio::Output<'static>; 5],
    // For exercise 2
    direction: Direction,
}

impl Dial {
    pub fn new(
        row_1: Row1Pin,
        row_2: Row2Pin,
        row_3: Row3Pin,
        row_4: Row4Pin,
        row_5: Row5Pin,
        col_1: Col1Pin,
        col_2: Col2Pin,
        col_3: Col3Pin,
        col_4: Col4Pin,
        col_5: Col5Pin,
    ) -> Self {
        let rows = [
            row_1.degrade(),
            row_2.degrade(),
            row_3.degrade(),
            row_4.degrade(),
            row_5.degrade(),
        ]
        .map(|pin| gpio::Output::new(pin, gpio::Level::Low, gpio::OutputDrive::Standard));
        let cols = [
            col_1.degrade(),
            col_2.degrade(),
            col_3.degrade(),
            col_4.degrade(),
            col_5.degrade(),
        ]
        .map(|pin| gpio::Output::new(pin, gpio::Level::High, gpio::OutputDrive::Standard));

        Self {
            rows,
            cols,
            direction: Direction::None,
        }
    }

    pub fn clear(&mut self) {
        self.rows.iter_mut().for_each(|r| r.set_low());
        self.cols.iter_mut().for_each(|c| c.set_high());
    }

    pub fn set_light_direction(&mut self, dir: Direction) {
        let (row, col) = dir.led_index();
        self.clear();
        self.rows[row].set_high();
        self.cols[col].set_low();
    }

    /// Operate the dial autonomously. This function is useful for running
    /// in a separate task.
    ///
    /// Useful for exercise 8.1.2
    pub async fn run(
        mut self,
        receiver: embassy_sync::channel::Receiver<'_, NoopRawMutex, Direction, 4>,
    ) -> ! {
        use embassy_time::{Duration, Ticker};
        let mut ticker = Ticker::every(Duration::from_millis(500));
        let mut is_on = false;
        loop {
            match select(receiver.receive(), ticker.next()).await {
                Either::First(new_dir) => {
                    if self.direction == new_dir {
                        // The direction did not change so we do not need to do anything
                        continue;
                    }

                    rtt_target::rprintln!("Setting direction to {:?}", new_dir);
                    self.direction = new_dir;
                }
                Either::Second(()) => is_on = !is_on,
            }

            if is_on {
                self.set_light_direction(self.direction);
            } else {
                self.clear();
            }
        }
    }
}

/// Sets up a channel over which [Direction]s can be sent and received.
/// Useful for exercise 8.1.2
pub fn dir_channel() -> (
    embassy_sync::channel::Receiver<'static, NoopRawMutex, Direction, 4>,
    embassy_sync::channel::Sender<'static, NoopRawMutex, Direction, 4>,
) {
    static DIR_CHANNEL: embassy_sync::once_lock::OnceLock<
        embassy_sync::channel::Channel<NoopRawMutex, Direction, 4>,
    > = OnceLock::new();

    let chan = DIR_CHANNEL.get_or_init(|| embassy_sync::channel::Channel::new());
    (chan.receiver(), chan.sender())
}
