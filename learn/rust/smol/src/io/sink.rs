use super::*;
use std::{io, pin, task};
/// [smol::io::sink]
pub fn sink() -> Sink {
    Sink
}
/// [smol::io::Sink]
pub struct Sink;
impl AsyncWrite for Sink {
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        task::Poll::Ready(Ok(buf.len()))
    }
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        task::Poll::Ready(Ok(()))
    }
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        task::Poll::Ready(Ok(()))
    }
}
