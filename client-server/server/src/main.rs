extern crate rand;
extern crate mio;
#[cfg(test)] #[macro_use] extern crate hamcrest;

mod proto;
mod tcp;

fn main() {
    let server = tcp::async::Server::new("0.0.0.0:3000");
    server.run();
}
