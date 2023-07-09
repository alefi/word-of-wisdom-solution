use std::env;

const DEFAULT_PORT: u16 = 4000;

pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn parse() -> Result<Config, &'static str> {
        let port: u16 = match env::var("PORT") {
            Ok(port_number) => port_number.parse().unwrap(),
            _ => DEFAULT_PORT,
        };

        Ok(Config { port })
    }
}
