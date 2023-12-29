use rustress::client::{Client, ClientConfig};

fn main() {
    let client_config = ClientConfig::default();

    Client::new(client_config).run();
}
