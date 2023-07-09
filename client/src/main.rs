use lib::{compute_challenge, disconnect};
use std::io::Read;
use std::net::{Shutdown, SocketAddr, TcpStream, ToSocketAddrs};
use word_of_wisdom_shared::{config::Config, parse_reply, send_message};

fn main() {
    let config = Config::parse().unwrap();
    let mut addr = format!("{}:{}", config.server_host, config.port)
        .to_socket_addrs()
        .unwrap();
    let addr: SocketAddr = addr.next().unwrap();

    let mut stream: TcpStream = TcpStream::connect(addr).unwrap();
    let mut buf: [u8; 256] = [0; 256];

    // Since we know the server would ask for a challenge, we're going to ask for a resource.
    send_message(&mut stream, String::from("preflight"));

    while match stream.read(&mut buf) {
        Ok(size) => {
            let message = parse_reply(&buf, size);

            if message.starts_with("quote:") {
                // Completed because of we've just received a demanded quote.
                // The receiver method has already printed the content out.
                false
            } else {
                send_challenge(&mut stream, message);
                true
            }
        }
        Err(_) => {
            println!("An error occurred, disconnecting");
            disconnect(&stream, Shutdown::Both);
            false
        }
    } {}
}

fn send_challenge(stream: &mut TcpStream, resource: String) {
    let challenge = compute_challenge(&resource);
    let message = format!("{}:{}", resource, challenge);
    send_message(stream, message);
}
