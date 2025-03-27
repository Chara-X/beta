use super::*;
use std::net;
/// [russh::server::Server]
pub trait Server {
    /// [russh::server::Handler]
    type Handler: Handler + Send + 'static;
    /// [russh::server::Server::new_client]
    fn new_client(&mut self, peer_addr: Option<net::SocketAddr>) -> Self::Handler;
}
