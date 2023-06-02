use crate::{cmd::CommandParser, serial::TxPort};
use clap::Parser;
use format::DeviceToServer;
use serialport::{SerialPortType, UsbPortInfo};
use std::thread;

mod cmd;
mod serial;

fn handle_message(msg: DeviceToServer) {
    println!("Got message: {:?}", msg);
    let DeviceToServer {
        led_status,
        said_hello,
    } = msg;
    if said_hello {
        println!("Device said hello!");
    }

    if let Some((led_id, enabled)) = led_status {
        let status = match enabled {
            true => "on",
            false => "off",
        };
        println!("Led {} status: {}", led_id, status);
    }
    // TODO, do cool stuff with the message that just came in.
}

/// Starts a simple REPL with which you can
/// send commands to the device. Take a look
/// in `cmd.rs` if you want to implement you own command
fn repl<const N: usize>(mut tx_port: TxPort<N>) {
    use crate::cmd::ParseError::*;
    use std::io::BufRead;

    let stdin = std::io::stdin();
    println!("Welcome to the device Commander! Please enter your command and press Enter");
    let mut lines = stdin.lock().lines();
    loop {
        if let Some(line) = lines.next() {
            match CommandParser::parse(&line.unwrap()) {
                Ok(cmd) => {
                    let msg = cmd.build_message();

                    tx_port.write_message(&msg).unwrap();
                    println!("Command sent!");
                }
                Err(CommandNotFound) => eprintln!("Error: Command not found"),
                Err(InvalidArgs) => eprintln!("Error: Command arguments invalid"),
            }
        } else {
            break;
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, help = "The path to the serial port to listen to")]
    port: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(port_name) = args.port {
        run(&port_name)
    } else {
        eprintln!("Please specify port as the first argument. For help, run with --help");
        eprintln!();
        print_available_ports();
    }
}

fn run(port_name: &str) {
    let port = serial::SerialPort::new(port_name.to_owned());

    match port {
        Ok(port) => {
            let (tx_port, mut rx_port): (TxPort<32>, _) = port.split();

            // Start a new thread on which we listen for data from the serial port
            let rx_thread = thread::spawn(move || rx_port.run_read_task::<_, 32>(handle_message));

            repl(tx_port);

            rx_thread.join().unwrap();
        }
        Err(e) => {
            eprintln!("Error opening serial port {}: {}", port_name, e);
            eprintln!();
            print_available_ports();
        }
    }
}

/// Lists available ports in stdout
fn print_available_ports() {
    println!("Available ports (listing USB only):");
    for port in serialport::available_ports().unwrap() {
        match (port.port_name, port.port_type) {
            (
                port_name,
                SerialPortType::UsbPort(UsbPortInfo {
                    vid,
                    pid,

                    manufacturer,
                    ..
                }),
            ) => {
                let manufacturer = manufacturer.unwrap_or_default();
                eprintln!(
                    "\t - {} (Vendor ID: {:#x}; Product ID: {:#x}; Manufacturer: {})",
                    port_name, vid, pid, manufacturer,
                );
            }
            _ => {} // Ignore other types
        }
    }
}
