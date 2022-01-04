use std::io::Read;
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;

use serde_derive::Deserialize;

#[serde(default)]
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub ip: String,
    pub port: u16,
    pub thread_count: u8,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            ip: String::from("127.0.0.1"),
            port: 8080,
            thread_count: 1,
        }
    }
}

pub struct Server {
    config: ServerConfig,
}

fn to_socket_addr(ip: &str, port: u16) -> SocketAddr {
    let ip_addr = IpAddr::from_str(ip).unwrap();
    SocketAddr::new(ip_addr, port)
}

impl Server {
    pub fn new(config: ServerConfig) -> Server {
        Server { config }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let socket_addr = to_socket_addr(&self.config.ip, self.config.port);
        let listener = TcpListener::bind(socket_addr)?;
        for stream in listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }

    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0 as u8; 100];
        match stream.read(&mut buffer) {
            Ok(size) => {
                match std::str::from_utf8(&buffer[0..size]) {
                    Ok(val) => {
                        println!("Read {}", val);
                    }
                    Err(e) => {
                        println!("Error during converting to string: {}", e)
                    }
                }
            }
            Err(e) => {
                println!("Error during reading to buffer: {}", e)
            }
        }
    }
}