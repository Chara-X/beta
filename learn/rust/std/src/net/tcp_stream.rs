use super::*;
use std::io;
/// [std::net::TcpStream]
pub struct TcpStream();
impl TcpStream {
    /// [std::net::TcpStream::connect]
    pub fn connect<A: ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        todo!()
    }
    /// [std::net::TcpStream::local_addr]
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        todo!()
    }
    /// [std::net::TcpStream::peer_addr]
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        todo!()
    }
    /// [std::net::TcpStream::nodelay]
    pub fn nodelay(&self) -> io::Result<bool> {
        todo!()
    }
    /// [std::net::TcpStream::set_nodelay]
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        todo!()
    }
    /// [std::net::TcpStream::set_nonblocking]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        todo!()
    }
    /// [std::net::TcpStream::peek]
    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}
