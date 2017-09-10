/// Protocol utilities.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use rand;

type AddressList = Vec<SocketAddr>;

struct Response {
    // Length = x, each SocketAddr is a random IP and a random port.
    addr: AddressList,
}

impl Response {
    fn new(addr_count: usize) -> Response {
        Response {
            addr: random_addrsv4(addr_count),
        }
    }
}

fn random_addrsv4(count: usize) -> AddressList {
    let mut addrs: AddressList = Vec::with_capacity(count);
    for _ in 0..count {
        addrs.push(random_addrv4());
    }
    addrs
}

fn random_addrv4() -> SocketAddr {
    // TODO: Generate only valid IP addresses.
    let (n1, n2, n3, n4) = rand::random::<(u8, u8, u8, u8)>();
    let port = rand::random::<u16>();
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(n1, n2, n3, n4)), port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest::prelude::*;

    #[test]
    fn random_addrsv4_returns_vector_with_specified_size() {
        let addrs = random_addrsv4(4);

        assert_that!(addrs.len(), is(equal_to(4)));
    }
}
