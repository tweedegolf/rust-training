#![allow(unused_doc_comments)]
/// 0. Verify this program works by:
/// - Run this binary with `cargo run`
/// - Run a client with `cargo run --bin client` or `nc 127.0.0.1 8080`

/// 1. Remove me!
use std::{
    io::{Read, Write},
    net::TcpListener,
};

/// 2. Uncomment us
// use tokio::net::TcpListener;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// 3. Uncomment me & follow the compiler (down the rabbit hole)
// #[tokio::main]
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    loop {
        let (mut socket, _) = listener.accept()?;

        /// 4. Replace std::thread with tokio::task & follow the compiler (just a bit deeper)
        std::thread::spawn(move || {
            // tokio::task::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf) {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                println!("Got a message with {n} bytes... gonna send that right back");

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]) {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

/// 5. Test the server still works the same way
#[allow(dead_code)]
fn done() {}
