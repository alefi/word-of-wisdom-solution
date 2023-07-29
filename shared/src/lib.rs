use std::io::{Result, Write};
use std::net::TcpStream;

pub mod config;

pub fn parse_reply(buf: &[u8], size: usize) -> String {
    let message: String = String::from_utf8_lossy(&buf[..size]).into();

    println!("<< {}", message);
    message
}

pub fn send_message(stream: &mut TcpStream, message: String) -> Result<()> {
    stream.write_all(message.as_bytes()).and({
        stream.flush().unwrap();
        println!(">> {}", message);
        Ok(())
    })
}
