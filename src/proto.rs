/// Protocol utilities.
/// Currently only IPv4 addresses are supported.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;

use rand;

type AddressList = Vec<SocketAddr>;

pub struct Response {
    // Length = x, each SocketAddr is a random IP and a random port.
    addr: AddressList,
}

impl Response {
    // TODO: support IPv6 too.
    pub fn new(addr_count: usize) -> Response {
        Response {
            addr: random_addrsv4(addr_count),
        }
    }

    pub fn from_bytes(data: &[u8]) -> Option<Response> {
        // TODO
        None
    }

    // TODO: use smth like https://github.com/TyOverby/bincode for more
    // robust (de)serialization.
    /// Serializes addresses to bytes where first two bytes is the number
    /// of addresses then every other 6 bytes is an address.
    pub fn into_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(2 + self.addr.len() * 6);
        bytes.extend(big_endian::u16_to_bytes(self.addr.len() as u16).iter());
        // sloooow...
        for a in &self.addr {
            bytes.extend(ipv4_to_bytes(a));
        }
        bytes
    }
}

/// Serializes address to bytes in big endian order.
fn ipv4_to_bytes(addr: &SocketAddr) -> Vec<u8> {
    let mut data: Vec<u8> = match addr.ip() {
        IpAddr::V4(ip4) => ip4.octets().to_vec(),
        _ => panic!("IPv6 is unsupported"),
    };
    let port = addr.port();
    data.extend(big_endian::u16_to_bytes(port).iter());
    data
}

mod big_endian {
    pub fn u16_to_bytes(data: u16) -> [u8; 2] {
        [(data >> 8) as u8, data as u8 & 0xff]
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

    #[test]
    fn ipv4_to_bytes_returns_6_bytes_representing_ip_address_with_port() {
        let addr = SocketAddr::from_str("1.2.3.4:10").unwrap();

        let bytes = ipv4_to_bytes(&addr);

        assert_that!(&bytes, contains(vec![1, 2, 3, 4, 0, 10]).in_order());
    }

    // Trying to emulate nesting what "stainless" lib already does.
    mod response {
        use super::*;

        #[test]
        fn into_bytes_returns_array_where_first_two_bytes_are_array_size() {
            let resp = Response::new(10);

            let bytes = resp.into_bytes();

            assert_that!(bytes[0], is(equal_to(0)));
            assert_that!(bytes[1], is(equal_to(10)));
        }
    }
}
