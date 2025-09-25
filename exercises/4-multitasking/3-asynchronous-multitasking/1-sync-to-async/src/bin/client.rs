use std::io::{Read, stdin, Write};
use std::net::TcpStream;

fn main() -> Result<(), std::io::Error>{
    let mut sock = TcpStream::connect("127.0.0.1:8080")?;

    println!("Connected! Type something to send to the server:");

    for line in stdin().lines() {
        let line = line?;

        println!("Sending: {line}");
        sock.write_all(line.as_bytes())?;

        let mut response = vec![0; 1024];
        let received_bytes = sock.read(&mut response)?;
        println!("Received: {}", String::from_utf8_lossy(&response[..received_bytes]));
    }

    Ok(())
}