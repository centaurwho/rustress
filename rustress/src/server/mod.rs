use std::io::Read;
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;

use crate::threadpool::pool::{ThreadPool, ThreadPoolFactory};
use crate::{MsgFormat, NetworkProt};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub ip: String,
    pub ports: Vec<u16>,
    pub accepted_protocols: Vec<NetworkProt>,
    pub accepted_formats: Vec<MsgFormat>,
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        ServerConfig {
            ip: String::from("127.0.0.1"),
            ports: vec![8080, 8081],
            accepted_protocols: Vec::new(),
            accepted_formats: Vec::new(),
        }
    }
}

pub struct Server {
    config: ServerConfig,
    pool: ThreadPool,
}

fn to_socket_addr(ip: &str, port: u16) -> SocketAddr {
    let ip_addr = IpAddr::from_str(ip).unwrap();
    SocketAddr::new(ip_addr, port)
}

impl Server {
    pub fn new(config: ServerConfig) -> Server {
        let pool = ThreadPoolFactory::new_fixed_sized(config.ports.len());

        Server { config, pool }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        for port in &self.config.ports {
            let client_handler = ClientHandler {};
            let socket_addr = to_socket_addr(&self.config.ip, *port);
            self.pool.execute(move || {
                let listener = TcpListener::bind(socket_addr).unwrap();
                for stream in listener.incoming() {
                    client_handler.handle_client(stream.unwrap());
                }
            })
        }
        Ok(())
    }
}

struct ClientHandler;

impl ClientHandler {
    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0u8; 100];
        match stream.read(&mut buffer) {
            Ok(size) => match std::str::from_utf8(&buffer[0..size]) {
                Ok(val) => {
                    println!("Read {}", val);
                }
                Err(e) => {
                    println!("Error during converting to string: {}", e)
                }
            },
            Err(e) => {
                println!("Error during reading to buffer: {}", e)
            }
        }
    }
}
