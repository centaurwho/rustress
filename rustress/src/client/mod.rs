use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;

use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ClientConfig {
    pub server_ip: String,
    pub server_port: u16,
    // TODO: Add a constructor and check ip and port values in it
}

impl Default for ClientConfig {
    fn default() -> ClientConfig {
        ClientConfig {
            server_ip: String::from("127.0.0.1"),
            server_port: 8080,
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
        Client {
            config
        }
    }

    pub fn run(&self) {
        let socket_addr = to_socket_addr(&self.config.server_ip, self.config.server_port);
        match TcpStream::connect(socket_addr) {
            Ok(mut stream) => {
                println!("Connected to server: {}", socket_addr);
                let msg = b"Hako";
                stream.write(msg).unwrap();
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}