use super::*;
use std::net;
/// [net::SocketAddr]
pub enum SocketAddr {
    /// [net::SocketAddr::V4]
    V4(SocketAddrV4),
    /// [net::SocketAddr::V6]
    V6(net::SocketAddrV6),
}
impl SocketAddr {
    /// [net::SocketAddr::new]
    pub const fn new(ip: IpAddr, port: u16) -> SocketAddr {
        todo!()
    }
    /// [net::SocketAddr::ip]
    pub const fn ip(&self) -> IpAddr {
        todo!()
    }
    /// [net::SocketAddr::set_ip]
    pub fn set_ip(&mut self, new_ip: IpAddr) {
        todo!()
    }
    /// [net::SocketAddr::port]
    pub const fn port(&self) -> u16 {
        todo!()
    }
    /// [net::SocketAddr::set_port]
    pub fn set_port(&mut self, new_port: u16) {
        todo!()
    }
}
/// [net::SocketAddrV4]
pub struct SocketAddrV4 {}
