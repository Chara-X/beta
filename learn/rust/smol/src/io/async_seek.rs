use std::{io, pin, task};
/// [smol::io::AsyncSeek]
pub trait AsyncSeek {
    /// [smol::io::AsyncSeek::poll_seek]
    fn poll_seek(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        pos: io::SeekFrom,
    ) -> task::Poll<Result<u64, io::Error>>;
}
