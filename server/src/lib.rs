use std::io::Read;
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;

#[derive(Debug)]
pub struct ServerConfig {
    pub address: SocketAddr,
    thread_count: u8,
}

impl ServerConfig {
    pub fn new(ip: &str, port: u16, thread_count: u8) -> ServerConfig {
        let ip_addr = IpAddr::from_str(ip).unwrap();
        ServerConfig {
            address: SocketAddr::new(IpAddr::from(ip_addr), port),
            thread_count,
        }
    }
}

pub struct Server {
    config: ServerConfig,
}

impl Server {
    pub fn new(config: ServerConfig) -> Server {
        Server { config }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let listener = TcpListener::bind(self.config.address)?;
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