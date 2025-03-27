use std::{io, pin, task};
/// [tokio::io::AsyncWrite]
pub trait AsyncWrite {
    /// [tokio::io::AsyncWrite::poll_write]
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>>;
    /// [tokio::io::AsyncWrite::poll_flush]
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>>;
    /// [tokio::io::AsyncWrite::poll_shutdown]
    fn poll_shutdown(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>>;
}
