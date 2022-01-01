use std::io::Write;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;

pub struct ClientConfig {
    server_address: SocketAddr,
}

impl ClientConfig {
    pub fn new(server_ip: &str, server_port: u16) -> ClientConfig {
        let ip_addr = IpAddr::from_str(server_ip).unwrap();
        ClientConfig {
            server_address: SocketAddr::new(IpAddr::from(ip_addr), server_port),
        }
    }
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
        match TcpStream::connect(self.config.server_address) {
            Ok(mut stream) => {
                println!("Connected to server: {}", self.config.server_address);
                let msg = b"Hako";
                stream.write(msg).unwrap();
            }
            Err(e) => {
                println!("Failed to connect: {}", e);
            }
        }
    }
}