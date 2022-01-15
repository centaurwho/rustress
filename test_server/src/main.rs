use rustress::server::{Server, ServerConfig};

fn main() {
    let server_config = ServerConfig::default();
    Server::new(server_config).run().unwrap();
}
