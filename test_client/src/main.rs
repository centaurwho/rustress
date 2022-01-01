use client::{ClientConfig, Client};

fn main() {
    let client_config = ClientConfig::default();

    Client::new(client_config).run();
}