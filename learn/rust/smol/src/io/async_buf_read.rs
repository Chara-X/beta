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
impl<T> AsyncBufRead for &mut T
where
    T: AsyncBufRead + Unpin + ?Sized,
{
    fn poll_fill_buf(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<&[u8], io::Error>> {
        pin::Pin::new(&mut **self.get_mut()).poll_fill_buf(cx)
    }
    fn consume(self: pin::Pin<&mut Self>, amt: usize) {
        pin::Pin::new(&mut **self.get_mut()).consume(amt)
    }
}
/// [smol::io::AsyncBufReadExt]
pub trait AsyncBufReadExt: AsyncBufRead {
    /// [smol::io::AsyncBufReadExt::fill_buf]
    fn fill_buf(&mut self) -> FillBuf<'_, Self>
    where
        Self: Unpin,
    {
        FillBuf { reader: Some(self) }
    }
}
impl<R: AsyncBufRead + ?Sized> AsyncBufReadExt for R {}
/// [smol::io::FillBuf]
pub struct FillBuf<'a, R>
where
    R: ?Sized,
{
    reader: Option<&'a mut R>,
}
impl<'a, R> Future for FillBuf<'a, R>
where
    R: AsyncBufRead + Unpin + ?Sized,
{
    type Output = Result<&'a [u8], io::Error>;
    fn poll(mut self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let this = &mut *self;
        let reader = this.reader.take().unwrap();
        match pin::Pin::new(&mut *reader).poll_fill_buf(cx) {
            task::Poll::Ready(Ok(_)) => match pin::Pin::new(reader).poll_fill_buf(cx) {
                task::Poll::Ready(Ok(slice)) => task::Poll::Ready(Ok(slice)),
                _ => panic!(),
            },
            task::Poll::Ready(Err(err)) => task::Poll::Ready(Err(err)),
            task::Poll::Pending => {
                this.reader = Some(reader);
                task::Poll::Pending
            }
        }
    }
}
