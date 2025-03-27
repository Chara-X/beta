use super::*;
use tokio::sync::oneshot;
/// [russh::server::Session]
pub struct Session {}
impl Session {
    /// [russh::server::Session::config]
    pub fn config(&self) -> &Config {
        todo!()
    }
    /// [russh::server::Session::remote_sshid]
    pub fn remote_sshid(&self) -> &[u8] {
        todo!()
    }
    /// [russh::server::Session::request_success]
    pub fn request_success(&mut self) {
        todo!()
    }
    /// [russh::server::Session::request_failure]
    pub fn request_failure(&mut self) {
        todo!()
    }
    /// [russh::server::Session::disconnect]
    pub fn disconnect(
        &mut self,
        reason: russh::Disconnect,
        description: &str,
        language_tag: &str,
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::channel_open_session]
    pub fn channel_open_session(&mut self) -> Result<russh::ChannelId, russh::Error> {
        todo!()
    }
    /// [russh::server::Session::channel_open_direct_tcpip]
    pub fn channel_open_direct_tcpip(
        &mut self,
        host_to_connect: &str,
        port_to_connect: u32,
        originator_address: &str,
        originator_port: u32,
    ) -> Result<russh::ChannelId, russh::Error> {
        todo!()
    }
    /// [russh::server::Session::channel_open_forwarded_tcpip]
    pub fn channel_open_forwarded_tcpip(
        &mut self,
        connected_address: &str,
        connected_port: u32,
        originator_address: &str,
        originator_port: u32,
    ) -> Result<russh::ChannelId, russh::Error> {
        todo!()
    }
    /// [russh::server::Session::channel_open_failure]
    pub fn channel_open_failure(
        &mut self,
        channel: russh::ChannelId,
        reason: russh::ChannelOpenFailure,
        description: &str,
        language: &str,
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::tcpip_forward]
    pub fn tcpip_forward(
        &mut self,
        address: &str,
        port: u32,
        reply_channel: Option<oneshot::Sender<Option<u32>>>,
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::cancel_tcpip_forward]
    pub fn cancel_tcpip_forward(
        &mut self,
        address: &str,
        port: u32,
        reply_channel: Option<oneshot::Sender<bool>>,
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::channel_success]
    pub fn channel_success(&mut self, channel: russh::ChannelId) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::channel_failure]
    pub fn channel_failure(&mut self, channel: russh::ChannelId) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::data]
    pub fn data(
        &mut self,
        channel: russh::ChannelId,
        data: russh::CryptoVec,
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::exit_status_request]
    pub fn exit_status_request(
        &mut self,
        channel: russh::ChannelId,
        exit_status: u32,
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::close]
    pub fn close(&mut self, channel: russh::ChannelId) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::server::Session::flush]
    pub fn flush(&mut self) -> Result<(), russh::Error> {
        todo!()
    }
}
