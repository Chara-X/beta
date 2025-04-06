use super::*;
use std::{io, pin, sync, task};
/// [smol::io::split]
pub fn split<T>(stream: T) -> (ReadHalf<T>, WriteHalf<T>)
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let stream = sync::Arc::new(sync::Mutex::new(stream));
    (ReadHalf(stream.clone()), WriteHalf(stream))
}
/// [smol::io::ReadHalf]
pub struct ReadHalf<T>(sync::Arc<sync::Mutex<T>>);
impl<T> AsyncRead for ReadHalf<T>
where
    T: AsyncRead + Unpin,
{
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        pin::Pin::new(&mut *self.0.lock().unwrap()).poll_read(cx, buf)
    }
}
/// [smol::io::WriteHalf]
pub struct WriteHalf<T>(sync::Arc<sync::Mutex<T>>);
impl<T> AsyncWrite for WriteHalf<T>
where
    T: AsyncWrite + Unpin,
{
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        pin::Pin::new(&mut *self.0.lock().unwrap()).poll_write(cx, buf)
    }
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        pin::Pin::new(&mut *self.0.lock().unwrap()).poll_flush(cx)
    }
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        pin::Pin::new(&mut *self.0.lock().unwrap()).poll_close(cx)
    }
}
