use server::{ServerConfig, Server};

fn main() {
    let server_config = ServerConfig::default();
    Server::new(server_config).run().unwrap();
}
