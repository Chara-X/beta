use super::*;
use std::io;
use std::net;
/// [net::TcpListener]
pub struct TcpListener();
impl TcpListener {
    /// [net::TcpListener::bind]
    pub fn bind<A: ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        todo!()
    }
    /// [net::TcpListener::set_nonblocking]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        todo!()
    }
    /// [net::TcpListener::accept]
    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        todo!()
    }
    /// [net::TcpListener::incoming]
    pub fn incoming(&self) -> net::Incoming<'_> {
        todo!()
    }
}
