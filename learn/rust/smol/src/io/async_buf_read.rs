use super::*;
use std::{io, pin, task};
/// [smol::io::AsyncBufRead]
pub trait AsyncBufRead: AsyncRead {
    /// [smol::io::AsyncBufRead::poll_fill_buf]
    fn poll_fill_buf(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<&[u8], io::Error>>;
    /// [smol::io::AsyncBufRead::consume]
    fn consume(self: pin::Pin<&mut Self>, amt: usize);
}
