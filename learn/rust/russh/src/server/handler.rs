use super::*;
use crate::keys;
use russh::server;
/// [server::Handler]
pub trait Handler: Sized {
    /// [server::Handler::Error]
    type Error: From<russh::Error> + Send;
    /// [server::Handler::authentication_banner]
    fn authentication_banner(
        &mut self,
    ) -> impl Future<Output = Result<Option<String>, Self::Error>> + Send;
    /// [server::Handler::auth_publickey]
    fn auth_publickey(
        &mut self,
        user: &str,
        public_key: &keys::PublicKey,
    ) -> impl Future<Output = Result<server::Auth, Self::Error>> + Send;
    /// [server::Handler::auth_password]
    fn auth_password(
        &mut self,
        user: &str,
        password: &str,
    ) -> impl Future<Output = Result<server::Auth, Self::Error>> + Send;
    /// [server::Handler::auth_none]
    fn auth_none(
        &mut self,
        user: &str,
    ) -> impl Future<Output = Result<server::Auth, Self::Error>> + Send;
    /// [server::Handler::auth_succeeded]
    fn auth_succeeded(
        &mut self,
        session: &mut Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// [server::Handler::channel_open_session]
    fn channel_open_session(
        &mut self,
        channel: crate::Channel<server::Msg>,
        session: &mut Session,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
    /// [server::Handler::channel_open_direct_tcpip]
    fn channel_open_direct_tcpip(
        &mut self,
        channel: crate::Channel<server::Msg>,
        host_to_connect: &str,
        port_to_connect: u32,
        originator_address: &str,
        originator_port: u32,
        session: &mut Session,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
    /// [server::Handler::channel_open_forwarded_tcpip]
    fn channel_open_forwarded_tcpip(
        &mut self,
        channel: crate::Channel<server::Msg>,
        host_to_connect: &str,
        port_to_connect: u32,
        originator_address: &str,
        originator_port: u32,
        session: &mut Session,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
    /// [server::Handler::channel_open_confirmation]
    fn channel_open_confirmation(
        &mut self,
        id: russh::ChannelId,
        max_packet_size: u32,
        window_size: u32,
        session: &mut Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// [server::Handler::tcpip_forward]
    fn tcpip_forward(
        &mut self,
        address: &str,
        port: &mut u32,
        session: &mut Session,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
    /// [server::Handler::cancel_tcpip_forward]
    fn cancel_tcpip_forward(
        &mut self,
        address: &str,
        port: u32,
        session: &mut Session,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
    /// [server::Handler::env_request]
    fn env_request(
        &mut self,
        channel: russh::ChannelId,
        variable_name: &str,
        variable_value: &str,
        session: &mut Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// [server::Handler::pty_request]
    fn pty_request(
        &mut self,
        channel: russh::ChannelId,
        term: &str,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        modes: &[(russh::Pty, u32)],
        session: &mut Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// [server::Handler::shell_request]
    fn shell_request(
        &mut self,
        channel: russh::ChannelId,
        session: &mut Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// [server::Handler::exec_request]
    fn exec_request(
        &mut self,
        channel: russh::ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// [server::Handler::data]
    fn data(
        &mut self,
        channel: russh::ChannelId,
        data: &[u8],
        session: &mut Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// [server::Handler::channel_close]
    fn channel_close(
        &mut self,
        channel: russh::ChannelId,
        session: &mut Session,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
