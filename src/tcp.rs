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

pub mod async {
    use std::str::FromStr;
    use std::net::SocketAddr;
    use std::io::{Read, Write};

    use mio::*;
    use mio::tcp::{TcpListener, TcpStream};

    const SERVER: Token = Token(0);

    // TODO: implement server trait. UDP or other transport protocol servers
    // could be implemented in the future.
    pub struct Server {
        listener: TcpListener,
        poll: Poll,
    }

    impl Server {
        pub fn new(addr: &str) -> Server {
            let addr = SocketAddr::from_str(addr).unwrap();
            Server {
                listener: TcpListener::bind(&addr).unwrap(),
                poll: Poll::new().unwrap(),
            }
        }

        pub fn run(&self) {
            self.poll.register(
                &self.listener,
                SERVER,
                Ready::readable(),
                PollOpt::edge()
            ).unwrap();

            let mut events = Events::with_capacity(1024);
            loop {
                self.poll.poll(&mut events, None).unwrap();
                self.on_event(&mut events);
            }
        }

        fn on_event(&self, events: &mut Events) {
            for event in events.iter() {
                match event.token() {
                    SERVER => self.on_new_connection(),
                    _ => unreachable!(),
                }
            }
        }

        fn on_new_connection(&self) {
            // TODO: handle errors.
            let (mut client, addr) = self.listener.accept().unwrap();
            println!("> Client connected: {}", addr);
            // TODO: register client on event loop. It just happens that
            // on localhost when client is connected it's already
            // ready to be read from.
            self.read_request(&mut client)
        }

        fn read_request(&self, client: &mut TcpStream) {
            let mut buf = [0u8; 4096];
            let bytes_read = client.read(&mut buf)
                .expect("! Failed to read data from client.");

            let request = String::from_utf8(buf.to_vec())
                .expect("! Invalid request.");

            println!("> Request: {}", request);
        }
    }
}
