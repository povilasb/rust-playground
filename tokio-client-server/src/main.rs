#[macro_use]
extern crate unwrap;
extern crate tokio_core;
extern crate futures;
use futures::future;

use tokio_core::reactor::Core;

fn main() {
    let mut core = unwrap!(Core::new());
    let res: Result<(), ()> = core.run(future::lazy(|| {
        println!("{}", "running inside event loop!");
        Ok(())
    }));
}
