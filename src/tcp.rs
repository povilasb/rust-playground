use std::io::{Read, Write};
use std::net;

use proto;

pub struct Server {
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
        let resp = proto::Response::new(3);
        client.write(&resp.into_bytes()[..]);
    }
}
