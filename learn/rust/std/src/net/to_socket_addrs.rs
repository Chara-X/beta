use super::*;
use std::io;
/// [std::net::ToSocketAddrs]
pub trait ToSocketAddrs {
    /// [std::net::ToSocketAddrs::to_socket_addrs]
    type Iter: Iterator<Item = SocketAddr>;
    /// [std::net::ToSocketAddrs::to_socket_addrs]
    fn to_socket_addrs(&self) -> io::Result<Self::Iter>;
}
