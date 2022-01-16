use serde::Deserialize;
use toml::de::Error;

use crate::client::ClientConfig;
use crate::server::ServerConfig;

fn parse_config<'a, T>(contents: &'a str) -> T
    where T: Deserialize<'a> {

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

#[cfg(test)]
mod test {
    use std::fs;
    use super::*;

    #[test]
    fn parse_server_config_only_mandatory_fields() {
        let path = String::from("src/configparser/res/server_config.toml");

        let contents = fs::read_to_string(path).unwrap();
        println!("{}", contents);

        let server_config = parse_server_config(contents);

        println!("{:?}", server_config);
    }
}
