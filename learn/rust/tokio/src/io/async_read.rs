use super::*;
use std::{io, pin, task};
/// [tokio::io::AsyncRead]
pub trait AsyncRead {
    /// [tokio::io::AsyncRead::poll_read]
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> task::Poll<io::Result<()>>;
}
