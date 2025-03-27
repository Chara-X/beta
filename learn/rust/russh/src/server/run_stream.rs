use super::*;
use russh::server;
use std::sync;
use tokio::io;
/// [russh::server::run_stream]
pub async fn run_stream<H, R>(
    config: sync::Arc<Config>,
    stream: R,
    handler: H,
) -> Result<server::Handle, H::Error>
where
    H: Handler + Send + 'static,
    R: io::AsyncRead + io::AsyncWrite + Unpin + Send + 'static,
{
    todo!()
}
