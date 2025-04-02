use std::{io, pin, task};
/// [smol::io::AsyncRead]
pub trait AsyncRead {
    /// [smol::io::AsyncRead::poll_read]
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>>;
}
