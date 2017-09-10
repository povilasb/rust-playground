extern crate rand;
#[cfg(test)] #[macro_use] extern crate hamcrest;

use std::io::{Read, Write};
use std::net;

mod proto;

struct Server {
    listener: net::TcpListener,
}

impl Server {
    /// Construct server with given bind address, e.g. "0.0.0.0:3000".
    pub fn new(addr: &str) -> Server {
        Server {
            listener: net::TcpListener::bind(addr).unwrap(),
        }
    }

    pub fn run(&self) {
        for stream in self.listener.incoming() {
            self.handle_conn(stream.unwrap());
        }
    }

    fn handle_conn(&self, mut client: net::TcpStream) {
        println!("Connected {}", client.peer_addr().unwrap());
        let mut buf = [0u8; 4096];
        let bytes_read = client.read(&mut buf)
            .expect("Failed to read data from client");
        println!("{}", bytes_read);
        self.respond_to_client(&mut client);
    }

    fn respond_to_client(&self, client: &mut net::TcpStream) {
        client.write(&"echo".to_string().into_bytes()[..]);
    }
}

fn main() {
    let server = Server::new("0.0.0.0:3000");
    server.run();
}
