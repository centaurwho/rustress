use serde::de::DeserializeOwned;
use toml::de::Error;

use crate::client::ClientConfig;
use crate::server::ServerConfig;

fn parse_config<T>(contents: &str) -> T
where
    T: DeserializeOwned,
{
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
    use std::collections::HashMap;
    use std::fs;

    use crate::client::Flow;
    use crate::{MsgFormat, NetworkProt};

    use super::*;

    #[test]
    fn parse_server_config_correct_file() {
        let path = String::from("src/configparser/res/server_config.toml");

        let contents = fs::read_to_string(path).unwrap();
        let server_config = parse_server_config(contents);

        assert_eq!(server_config.ports, vec![9990, 9991]);
        assert_eq!(
            server_config.accepted_protocols,
            vec![NetworkProt::Tcp, NetworkProt::Udp]
        );
        assert_eq!(
            server_config.accepted_formats,
            vec![MsgFormat::Json, MsgFormat::Xml]
        );
    }

    #[test]
    fn parse_client_config_correct_file() {
        let path = String::from("src/configparser/res/client_config.toml");

        let contents = fs::read_to_string(path).unwrap();
        let client_config = parse_client_config(contents);

        println!("{:?}", client_config);

        assert_eq!(client_config.server_ip, "8.8.8.8");

        let mut map = HashMap::new();
        map.insert(
            String::from("tcp_flow"),
            Flow {
                name: String::from("Connection 1"),
                server_port: 8000,
                protocol: NetworkProt::Tcp,
                format: MsgFormat::Json,
            },
        );

        map.insert(
            String::from("udp_flow"),
            Flow {
                name: String::from("Connection 2"),
                server_port: 8001,
                protocol: NetworkProt::Udp,
                format: MsgFormat::Xml,
            },
        );

        assert_eq!(client_config.flows, map);
    }
}
