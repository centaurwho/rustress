use client::{ClientConfig, Client};

fn main() {
    let ip = "127.0.0.1";
    let port = 8080;

    let client_config = ClientConfig::new(ip, port);

    Client::new(client_config).run();
}