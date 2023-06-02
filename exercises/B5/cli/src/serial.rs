#![allow(dead_code)]
use std::{io, time::Duration};

use format::{DeviceToServer, ServerToDevice};
use postcard::accumulator::{CobsAccumulator, FeedResult};

// Wrapper around a serialport. Can be split up
// into a TxPort and an RxPort for use in separate tasks.
// The N parameter denotes the size of the message
// serialization buffer of the TxPort.
pub struct SerialPort<const N: usize> {
    port: Box<dyn serialport::SerialPort>,
}

impl<const N: usize> SerialPort<N> {
    pub fn new(port_name: String) -> Result<Self, serialport::Error> {
        let port = serialport::new(port_name, 115200)
            .timeout(Duration::from_millis(1000))
            .open()?;
        Ok(Self { port })
    }

    pub fn split(self) -> (TxPort<N>, RxPort) {
        let tx_port = TxPort::<N>::new(self.port.try_clone().unwrap());
        let rx_port = RxPort::new(self.port);
        (tx_port, rx_port)
    }
}

pub struct RxPort {
    port: Box<dyn serialport::SerialPort>,
}

impl RxPort {
    pub fn new(port: Box<dyn serialport::SerialPort>) -> Self {
        Self { port }
    }

    /// Starts listening for incoming  data on the serial port, and
    /// tries to parse incoming data. Calls `on_msg` when a new message
    /// was received and parsed sucessfully.
    pub fn run_read_task<F: Fn(DeviceToServer) -> (), const N: usize>(&mut self, on_msg: F) {
        let mut accumulator = CobsAccumulator::<32>::new();
        let mut serial_buf = [0u8; N];

        loop {
            let chunk_len = self
                .port
                .read(&mut serial_buf)
                .or_else(|e| {
                    if e.kind() == std::io::ErrorKind::TimedOut {
                        // Just a time out before
                        // the buffer is full, no need to worry
                        Ok(0)
                    } else {
                        Err(e)
                    }
                })
                .expect("Serial read error");

            let chunk = &serial_buf[..chunk_len];
            match accumulator.feed(chunk) {
                FeedResult::Consumed => {} // Do nothing
                FeedResult::OverFull(_) => eprintln!("Accumulator full, dropping contents"),
                FeedResult::DeserError(_) => eprintln!("Deserialize error, throwing away message"),
                FeedResult::Success { data, .. } => on_msg(data), // Handle message
            }
        }
    }
}

pub struct TxPort<const N: usize> {
    port: Box<dyn serialport::SerialPort>,
    buf: [u8; N],
}

impl<const N: usize> TxPort<N> {
    pub fn new(port: Box<dyn serialport::SerialPort>) -> Self {
        Self {
            port,
            buf: [0u8; N],
        }
    }

    // Write a message to the device. This method blocks while other transactions are going on
    pub fn write_message(&mut self, msg: &ServerToDevice) -> Result<(), io::Error> {
        while let 1.. = self.port.bytes_to_write().unwrap() {
            // There are still bytes awaiting transmission
            // Wait for current write task to finish
        }
        let msg = postcard::to_slice_cobs(msg, &mut self.buf).unwrap();
        self.port.write(&msg).map(|_| {})
    }
}
