use super::*;
use russh::client;
use std::sync;
use tokio::io;
/// [client::connect_stream]
pub async fn connect_stream<H, R>(
    config: sync::Arc<Config>,
    stream: R,
    handler: H,
) -> Result<Handle<H>, H::Error>
where
    H: client::Handler + Send + 'static,
    R: io::AsyncRead + io::AsyncWrite + Unpin + Send + 'static,
{
    todo!()
}
