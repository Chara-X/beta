use tokio::io;
/// [kube::api::Portforwarder]
pub struct Portforwarder {}
impl Portforwarder {
    /// [kube::api::Portforwarder::take_stream]
    pub fn take_stream(
        &mut self,
        port: u16,
    ) -> Option<impl io::AsyncRead + io::AsyncWrite + Unpin> {
        Some(io::empty())
    }
    /// [kube::api::Portforwarder::join]
    pub async fn join(self) -> Result<(), kube::Error> {
        todo!()
    }
    /// [kube::api::Portforwarder::abort]
    pub fn abort(&self) {
        todo!()
    }
}
