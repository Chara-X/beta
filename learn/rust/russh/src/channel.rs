use std::marker;
use tokio::io;
/// [russh::Channel]
pub struct Channel<Send: From<(russh::ChannelId, russh::ChannelMsg)>> {
    _data: marker::PhantomData<Send>,
}
impl<S: From<(russh::ChannelId, russh::ChannelMsg)> + Send + Sync + 'static> Channel<S> {
    /// [russh::Channel::id]
    pub fn id(&self) -> russh::ChannelId {
        todo!()
    }
    /// [russh::Channel::set_env]
    pub async fn set_env<A: Into<String>, B: Into<String>>(
        &self,
        want_reply: bool,
        variable_name: A,
        variable_value: B,
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::Channel::request_pty]
    pub async fn request_pty(
        &self,
        want_reply: bool,
        term: &str,
        col_width: u32,
        row_height: u32,
        pix_width: u32,
        pix_height: u32,
        terminal_modes: &[(russh::Pty, u32)],
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::Channel::request_shell]
    pub async fn request_shell(&self, want_reply: bool) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::Channel::exec]
    pub async fn exec<A: Into<Vec<u8>>>(
        &self,
        want_reply: bool,
        command: A,
    ) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::Channel::data]
    pub async fn data<R: io::AsyncRead + Unpin>(&self, data: R) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::Channel::wait]
    pub async fn wait(&mut self) -> Option<russh::ChannelMsg> {
        todo!()
    }
    /// [russh::Channel::close]
    pub async fn close(&self) -> Result<(), russh::Error> {
        todo!()
    }
    /// [russh::Channel::into_stream]
    pub fn into_stream(self) -> russh::ChannelStream<S> {
        todo!()
    }
}
