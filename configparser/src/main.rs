use std::fs::File;
use std::io::Write;

use toml::de::Error;
use toml::macros::Deserialize;

use client::ClientConfig;
use server::ServerConfig;

fn parse_config<'a, T>(contents: &'a str, def: Option<T>) -> T
    where T: Deserialize<'a> { // TODO: Decide lifetime of this
    let config: Result<T, Error> = toml::from_str(contents);
    match config {
        Ok(config) => config,
        Err(e) => {
            println!("Incorrect config for server: {}", e);
            match def {
                None => {
                    panic!("Quitting...")
                }
                Some(def) => {
                    println!("Using default config");
                    def
                }
            }
        }
    }
}

fn parse_server_config(contents: String) -> ServerConfig {
    parse_config(contents.as_str(), Some(ServerConfig::default()))
}

fn parse_client_config(contents: String) -> ClientConfig {
    parse_config(contents.as_str(), Some(ClientConfig::default()))
}

fn main() {
    // TODO: Use relative path if possible
    let server_config_file = String::from("configparser/res/server_config.toml");
    let client_config_file = String::from("configparser/res/client_config.toml");

    let contents = std::fs::read_to_string(server_config_file).unwrap();
    let server_config = parse_server_config(contents);
    println!("{:?}", server_config);

    let contents = std::fs::read_to_string(client_config_file).unwrap();
    let client_config = parse_client_config(contents);
    println!("{:?}", client_config);
}
