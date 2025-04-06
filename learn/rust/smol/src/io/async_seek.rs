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
/// [smol::io::AsyncSeekExt]
pub trait AsyncSeekExt: AsyncSeek {
    /// [smol::io::AsyncSeekExt::seek]
    fn seek(&mut self, pos: io::SeekFrom) -> SeekFuture<'_, Self>
    where
        Self: Unpin,
    {
        SeekFuture { seeker: self, pos }
    }
}
impl<R: AsyncSeek + ?Sized> AsyncSeekExt for R {}
/// [smol::io::SeekFuture]
pub struct SeekFuture<'a, R>
where
    R: ?Sized,
{
    seeker: &'a mut R,
    pos: io::SeekFrom,
}
impl<R> Future for SeekFuture<'_, R>
where
    R: AsyncSeek + Unpin + ?Sized,
{
    type Output = Result<u64, io::Error>;
    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let pos = self.pos;
        pin::Pin::new(&mut *self.seeker).poll_seek(cx, pos)
    }
}
