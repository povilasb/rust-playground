#[macro_use]
extern crate unwrap;
extern crate tokio_core;
extern crate futures;
use futures::future;

use std::net::SocketAddr;
use std::{thread, time};

use tokio_core::reactor::Core;
use tokio_core::net::{TcpListener, TcpStream};
use futures::Stream;

fn main() {
    let mut core = unwrap!(Core::new());

    let listener = unwrap!(TcpListener::bind(
        &unwrap!("0.0.0.0:5000".parse()),
        &core.handle(),
    ));
    let accept_conns = listener.incoming().for_each(|(client, addr)| {
        core.handle().spawn_fn(|| {
            on_connected(client, addr);
            Ok(())
        });
        Ok(())
    });

    unwrap!(core.run(accept_conns));
}

fn on_connected(client: TcpStream, addr: SocketAddr) {
    println!("Connected: {}", addr);
    thread::sleep(time::Duration::from_secs(5));
}
