use lib::handle_client;
use std::net::{SocketAddr, TcpListener};
use std::thread;
use word_of_wisdom_shared::config::Config;

fn main() {
    let config = Config::parse().unwrap();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener: TcpListener = TcpListener::bind(addr).unwrap();

    println!("Start listening on port:{}", addr.port());

    for tpc_stream in listener.incoming() {
        match tpc_stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error:{}", e);
            }
        }
    }
}
