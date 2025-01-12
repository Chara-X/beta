use std::io;
use std::net;
/// [net::TcpStream]
pub struct TcpStream();
impl TcpStream {
    /// [net::TcpStream::connect]
    pub fn connect<A: net::ToSocketAddrs>(addr: A) -> io::Result<TcpStream> {
        todo!()
    }
    /// [net::TcpStream::local_addr]
    pub fn local_addr(&self) -> io::Result<net::SocketAddr> {
        todo!()
    }
    /// [net::TcpStream::peer_addr]
    pub fn peer_addr(&self) -> io::Result<net::SocketAddr> {
        todo!()
    }
}
/// [net::TcpListener]
pub struct TcpListener();
impl TcpListener {
    /// [net::TcpListener::bind]
    pub fn bind<A: net::ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
        todo!()
    }
    /// [net::TcpListener::accept]
    pub fn accept(&self) -> io::Result<(TcpStream, net::SocketAddr)> {
        todo!()
    }
}
