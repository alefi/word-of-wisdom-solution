use hashcash::Stamp;
use std::net::{Shutdown, TcpStream};

pub fn compute_challenge(resource: &str) -> String {
    Stamp::with_resource_and_bits(resource, 16, false)
        .unwrap()
        .to_string()
}

pub fn disconnect(stream: &TcpStream, how: Shutdown) {
    stream.shutdown(how).unwrap()
}
