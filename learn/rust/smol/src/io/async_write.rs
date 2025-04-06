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
impl<T> AsyncWrite for &mut T
where
    T: AsyncWrite + Unpin + ?Sized,
{
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        pin::Pin::new(&mut **self.get_mut()).poll_write(cx, buf)
    }
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        pin::Pin::new(&mut **self.get_mut()).poll_flush(cx)
    }
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        pin::Pin::new(&mut **self.get_mut()).poll_close(cx)
    }
}
/// [smol::io::AsyncWriteExt]
pub trait AsyncWriteExt: AsyncWrite {
    /// [smol::io::AsyncWriteExt::write]
    fn write<'a>(&'a mut self, buf: &'a [u8]) -> WriteFuture<'a, Self>
    where
        Self: Unpin,
    {
        WriteFuture { writer: self, buf }
    }
    /// [smol::io::AsyncWriteExt::flush]
    fn flush(&mut self) -> FlushFuture<'_, Self>
    where
        Self: Unpin,
    {
        FlushFuture { writer: self }
    }
}
impl<W: AsyncWrite + ?Sized> AsyncWriteExt for W {}
/// [smol::io::WriteFuture]
pub struct WriteFuture<'a, W>
where
    W: Unpin + ?Sized,
{
    writer: &'a mut W,
    buf: &'a [u8],
}
impl<W> Future for WriteFuture<'_, W>
where
    W: AsyncWrite + Unpin + ?Sized,
{
    type Output = Result<usize, io::Error>;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let Self { writer, buf } = self.get_mut();
        pin::Pin::new(writer).poll_write(cx, buf)
    }
}
/// [smol::io::FlushFuture]
pub struct FlushFuture<'a, W>
where
    W: Unpin + ?Sized,
{
    writer: &'a mut W,
}
impl<W> Future for FlushFuture<'_, W>
where
    W: AsyncWrite + Unpin + ?Sized,
{
    type Output = Result<(), io::Error>;
    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        pin::Pin::new(&mut *self.writer).poll_flush(cx)
    }
}
