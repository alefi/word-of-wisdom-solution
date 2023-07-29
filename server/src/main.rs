use lib::handle_stream;
use std::net::{SocketAddr, TcpListener};
use word_of_wisdom_shared::config::Config;

fn main() {
    let config = Config::parse().unwrap();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let listener: TcpListener = TcpListener::bind(addr).unwrap();

    println!("Start listening on port:{}", addr.port());
    handle_stream(listener)
}
