use super::*;
use std::{io, pin, task};
/// [smol::io::repeat]
pub fn repeat(byte: u8) -> Repeat {
    Repeat { byte }
}
/// [smol::io::Repeat]
pub struct Repeat {
    byte: u8,
}
impl AsyncRead for Repeat {
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        for b in &mut *buf {
            *b = self.byte;
        }
        task::Poll::Ready(Ok(buf.len()))
    }
}
