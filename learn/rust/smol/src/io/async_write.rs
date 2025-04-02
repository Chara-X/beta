use std::{io, pin, task};
/// [smol::io::AsyncWrite]
pub trait AsyncWrite {
    /// [smol::io::AsyncWrite::poll_write]
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>>;
    /// [smol::io::AsyncWrite::poll_flush]
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>>;
    /// [smol::io::AsyncWrite::poll_close]
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>>;
}
