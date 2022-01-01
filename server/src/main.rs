use server::{ServerConfig, Server};

fn main() {
    let ip = "127.0.0.1";
    let port = 8080;
    let thread_count = 1;

    let server_config = ServerConfig::new(ip, port, thread_count);

    println!("{}", server_config.address);
    Server::new(server_config).run().unwrap();
}
