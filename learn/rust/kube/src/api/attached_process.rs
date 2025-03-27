use tokio::io;
/// [kube::api::AttachedProcess]
pub struct AttachedProcess {}
impl AttachedProcess {
    /// [kube::api::AttachedProcess::stdin]
    pub fn stdin(&mut self) -> Option<impl io::AsyncWrite + Unpin> {
        Some(io::empty())
    }
    /// [kube::api::AttachedProcess::stdout]
    pub fn stdout(&mut self) -> Option<impl io::AsyncRead + Unpin> {
        Some(io::empty())
    }
    /// [kube::api::AttachedProcess::stderr]
    pub fn stderr(&mut self) -> Option<impl io::AsyncRead + Unpin> {
        Some(io::empty())
    }
    /// [kube::api::AttachedProcess::join]
    pub async fn join(self) -> Result<(), kube::Error> {
        todo!()
    }
    /// [kube::api::AttachedProcess::abort]
    pub fn abort(&self) {
        todo!()
    }
}
