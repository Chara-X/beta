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
impl<T> AsyncRead for &mut T
where
    T: AsyncRead + Unpin + ?Sized,
{
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        pin::Pin::new(&mut **self.get_mut()).poll_read(cx, buf)
    }
}
/// [smol::io::AsyncReadExt]
pub trait AsyncReadExt: AsyncRead {
    /// [smol::io::AsyncReadExt::read]
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> ReadFuture<'a, Self>
    where
        Self: Unpin,
    {
        ReadFuture { reader: self, buf }
    }
}
impl<R: AsyncRead + ?Sized> AsyncReadExt for R {}
/// [smol::io::ReadFuture]
pub struct ReadFuture<'a, R>
where
    R: Unpin + ?Sized,
{
    reader: &'a mut R,
    buf: &'a mut [u8],
}
impl<R> Future for ReadFuture<'_, R>
where
    R: AsyncRead + Unpin + ?Sized,
{
    type Output = Result<usize, io::Error>;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let Self { reader, buf } = self.get_mut();
        pin::Pin::new(reader).poll_read(cx, buf)
    }
}
