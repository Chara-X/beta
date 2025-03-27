use crate::keys;
use russh::client;
use std::{marker, sync};
/// [client::Handle]
pub struct Handle<H: client::Handler> {
    _data: marker::PhantomData<H>,
}
impl<H: client::Handler> Handle<H> {
    /// [client::Handle::authenticate_publickey]
    pub async fn authenticate_publickey<U: Into<String>>(
        &mut self,
        user: U,
        key: sync::Arc<keys::PrivateKey>,
    ) -> Result<client::AuthResult, russh::Error> {
        todo!()
    }
    /// [client::Handle::authenticate_password]
    pub async fn authenticate_password<U: Into<String>, P: Into<String>>(
        &mut self,
        user: U,
        password: P,
    ) -> Result<client::AuthResult, russh::Error> {
        todo!()
    }
    /// [client::Handle::authenticate_none]
    pub async fn authenticate_none<U: Into<String>>(
        &mut self,
        user: U,
    ) -> Result<client::AuthResult, russh::Error> {
        todo!()
    }
    /// [client::Handle::channel_open_session]
    pub async fn channel_open_session(&self) -> Result<crate::Channel<client::Msg>, russh::Error> {
        todo!()
    }
    /// [client::Handle::channel_open_direct_tcpip]
    pub async fn channel_open_direct_tcpip<A: Into<String>, B: Into<String>>(
        &self,
        host_to_connect: A,
        port_to_connect: u32,
        originator_address: B,
        originator_port: u32,
    ) -> Result<crate::Channel<client::Msg>, russh::Error> {
        todo!()
    }
    /// [client::Handle::tcpip_forward]
    pub async fn tcpip_forward<A: Into<String>>(
        &mut self,
        address: A,
        port: u32,
    ) -> Result<u32, russh::Error> {
        todo!()
    }
    /// [client::Handle::disconnect]
    pub async fn disconnect(
        &self,
        reason: russh::Disconnect,
        description: &str,
        language_tag: &str,
    ) -> Result<(), russh::Error> {
        todo!()
    }
}
