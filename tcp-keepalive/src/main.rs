extern crate libc;
extern crate mio;
#[macro_use]
extern crate net_literals;
#[macro_use]
extern crate unwrap;

use mio::net::TcpStream;
use std::io;
use std::time::Duration;

#[cfg(not(target_os = "linux"))]
use other::set_keep_alive;
#[cfg(target_os = "linux")]
use unix::set_keep_alive;

fn main() {
    let stream = unwrap!(TcpStream::connect(&addr!("192.168.1.210:6000")));
    unwrap!(set_keep_alive(&stream, 1, 5, 5));
    println!("Press ENTER to exit");
    let _ = unwrap!(readln());
}

#[cfg(not(target_os = "linux"))]
mod other {
    use super::*;

    /// Do nothing on unknown systems.
    pub fn set_keep_alive(
        _stream: &TcpStream,
        _idle: u32,
        _interval: u32,
        _count: u32,
    ) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(target_os = "linux")]
mod unix {
    use super::*;
    use std::mem;
    use std::os::unix::io::{AsRawFd, RawFd};

    /// Set TCP keep alive options for a given socket.
    pub fn set_keep_alive(
        stream: &TcpStream,
        idle: u32,
        interval: u32,
        count: u32,
    ) -> io::Result<()> {
        stream.set_keepalive(Some(Duration::from_secs(u64::from(idle))))?;
        let fd = stream.as_raw_fd();
        set_ip_opt(fd, libc::TCP_KEEPINTVL, interval)?;
        set_ip_opt(fd, libc::TCP_KEEPCNT, count)?;
        Ok(())
    }

    /// Sets IP level option for a given socketlevel option for a given socket
    fn set_ip_opt(sock_fd: RawFd, opt: libc::c_int, val: u32) -> io::Result<()> {
        unsafe {
            let optval: libc::c_int = val as libc::c_int;
            let ret = libc::setsockopt(
                sock_fd,
                libc::IPPROTO_TCP,
                opt,
                &optval as *const _ as *const libc::c_void,
                mem::size_of_val(&optval) as libc::socklen_t,
            );
            if ret != 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }
}

fn readln() -> io::Result<String> {
    let mut ln = String::new();
    io::stdin().read_line(&mut ln)?;
    Ok(String::from(ln.trim()))
}
