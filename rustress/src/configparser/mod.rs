use serde::Deserialize;
use toml::de::Error;

use crate::client::ClientConfig;
use crate::server::ServerConfig;

fn parse_config<'a, T>(contents: &'a str) -> T
    where T: Deserialize<'a> { // TODO: Decide lifetime of this

    let config: Result<T, Error> = toml::from_str(contents);
    match config {
        Ok(config) => config,
        Err(e) => {
            panic!("Incorrect config for server: {}", e);
        }
    }
}

fn parse_server_config(contents: String) -> ServerConfig {
    parse_config(contents.as_str())
}

fn parse_client_config(contents: String) -> ClientConfig {
    parse_config(contents.as_str())
}
