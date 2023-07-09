use crate::pow::Pow;
use crate::quotes::get_random_quote;
use std::io::Read;
use std::net::{IpAddr, TcpStream};
use word_of_wisdom_shared::{parse_reply, send_message};

mod pow;
mod quotes;

pub fn extract_client_ip(stream: &TcpStream) -> IpAddr {
    match stream.peer_addr() {
        Ok(socket_addr) => socket_addr.ip(),
        Err(e) => panic!("Illegal TCP socket format {:#?}", e),
    }
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buf: [u8; 256] = [0; 256];

    // Using an IP address to distinct a client.
    // However, it could be lot of clients connected through a single IP address.
    let client_ip = extract_client_ip(&stream);

    while match stream.read(&mut buf) {
        Ok(0) => return, // EOF
        Ok(size) => {
            let message = parse_reply(&buf, size);

            match Pow::parse(message) {
                Ok(_) => {
                    let quote = get_random_quote();
                    let message = format!("quote:{}", quote);
                    send_message(&mut stream, message);
                }
                Err(_) => send_challenge_required(&mut stream, &client_ip),
            };

            true
        }
        Err(_) => {
            println!(
                "An error occurred, terminating connection with:{}",
                extract_client_ip(&stream)
            );
            false
        }
    } {}
}

fn send_challenge_required(stream: &mut TcpStream, client_ip: &IpAddr) {
    let pow_object: Pow = Pow::new(client_ip, None);
    let resource_hash = pow_object.resource.get_hash();
    send_message(stream, resource_hash.to_string());
}
