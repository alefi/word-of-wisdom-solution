use std::env;

const DEFAULT_PORT: u16 = 4000;
const DEFAULT_SERVER_HOST: &str = "127.0.0.1";

pub struct Config {
    pub port: u16,
    pub server_host: String,
}

impl Config {
    pub fn parse() -> Result<Config, &'static str> {
        let port: u16 = match env::var("PORT") {
            Ok(port_number) => port_number.parse().unwrap(),
            _ => DEFAULT_PORT,
        };

        let server_host: String =
            env::var("SERVER_HOST").unwrap_or(DEFAULT_SERVER_HOST.to_string());

        Ok(Config { port, server_host })
    }
}
