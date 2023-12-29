use serde_derive::Deserialize;
use std::collections::HashMap;
use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;

use crate::{MsgFormat, NetworkProt};

#[derive(Debug, Deserialize, Eq, PartialEq)]
#[serde(default)]
pub struct Flow {
    pub(crate) name: String,
    pub(crate) server_port: u16,
    pub(crate) protocol: NetworkProt,
    pub(crate) format: MsgFormat,
}

impl Default for Flow {
    fn default() -> Self {
        Flow {
            name: String::new(),
            server_port: 8000,
            protocol: NetworkProt::Tcp,
            format: MsgFormat::Json,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ClientConfig {
    pub server_ip: String,
    pub flows: HashMap<String, Flow>,
    // TODO: Add a constructor and check ip and port values in it
}

impl Default for ClientConfig {
    fn default() -> ClientConfig {
        ClientConfig {
            server_ip: String::from("127.0.0.1"),
            flows: HashMap::new(),
        }
    }
}

fn to_socket_addr(ip: &str, port: u16) -> SocketAddr {
    let ip_addr = IpAddr::from_str(ip).unwrap();
    SocketAddr::new(ip_addr, port)
}

pub struct Client {
    config: ClientConfig,
}

impl Client {
    pub fn new(config: ClientConfig) -> Client {
        Client { config }
    }

    pub fn run(&self) {
        for flow in self.config.flows.values() {
            let socket_addr = to_socket_addr(&self.config.server_ip, flow.server_port);
            match TcpStream::connect(socket_addr) {
                Ok(mut stream) => {
                    println!("Connected to server: {}", socket_addr);
                    let msg = b"Hako";
                    stream.write_all(msg).unwrap();
                }
                Err(e) => {
                    println!("Failed to connect: {}", e);
                }
            }
        }
    }
}
