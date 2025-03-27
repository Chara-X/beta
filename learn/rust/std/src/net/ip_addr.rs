use std::net;
/// [net::IpAddr]
pub enum IpAddr {
    /// [net::IpAddr::V4]
    V4(Ipv4Addr),
    /// [net::IpAddr::V6]
    V6(net::Ipv6Addr),
}
impl IpAddr {
    /// [net::IpAddr::is_unspecified]
    pub const fn is_unspecified(&self) -> bool {
        todo!()
    }
    /// [net::IpAddr::is_loopback]
    pub const fn is_loopback(&self) -> bool {
        todo!()
    }
    /// [net::IpAddr::is_multicast]
    pub const fn is_multicast(&self) -> bool {
        todo!()
    }
    /// [net::IpAddr::to_canonical]
    pub const fn to_canonical(&self) -> IpAddr {
        todo!()
    }
}
/// [net::Ipv4Addr]
pub struct Ipv4Addr {}
impl Ipv4Addr {
    /// [net::Ipv4Addr::BITS]
    pub const BITS: u32 = 32u32;
    /// [net::Ipv4Addr::UNSPECIFIED]
    pub const UNSPECIFIED: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
    /// [net::Ipv4Addr::LOCALHOST]
    pub const LOCALHOST: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
    /// [net::Ipv4Addr::BROADCAST]
    pub const BROADCAST: Ipv4Addr = Ipv4Addr::new(255, 255, 255, 255);
    /// [net::Ipv4Addr::new]
    pub const fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        todo!()
    }
    /// [net::Ipv4Addr::from_bits]
    pub const fn from_bits(bits: u32) -> Ipv4Addr {
        todo!()
    }
    /// [net::Ipv4Addr::octets]
    pub const fn octets(&self) -> [u8; 4] {
        todo!()
    }
    /// [net::Ipv4Addr::to_bits]
    pub const fn to_bits(self) -> u32 {
        todo!()
    }
    /// [net::Ipv4Addr::to_ipv6_mapped]
    pub const fn to_ipv6_mapped(&self) -> net::Ipv6Addr {
        todo!()
    }
}
