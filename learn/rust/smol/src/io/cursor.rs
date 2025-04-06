use super::*;
use std::{io, pin, task};
/// [smol::io::Cursor]
pub struct Cursor<T> {
    inner: io::Cursor<T>,
}
impl<T> Cursor<T> {
    /// [smol::io::Cursor::new]
    pub fn new(inner: T) -> Cursor<T> {
        Cursor {
            inner: io::Cursor::new(inner),
        }
    }
    /// [smol::io::Cursor::get_ref]
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }
    /// [smol::io::Cursor::get_mut]
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }
    /// [smol::io::Cursor::into_inner]
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}
impl<T> AsyncRead for Cursor<T>
where
    T: AsRef<[u8]> + Unpin,
{
    fn poll_read(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        task::Poll::Ready(io::Read::read(&mut self.inner, buf))
    }
}
impl AsyncWrite for Cursor<&mut [u8]> {
    fn poll_write(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        task::Poll::Ready(io::Write::write(&mut self.inner, buf))
    }
    fn poll_flush(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        task::Poll::Ready(io::Write::flush(&mut self.inner))
    }
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        self.poll_flush(cx)
    }
}
impl AsyncWrite for Cursor<&mut Vec<u8>> {
    fn poll_write(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        task::Poll::Ready(io::Write::write(&mut self.inner, buf))
    }
    fn poll_flush(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        task::Poll::Ready(io::Write::flush(&mut self.inner))
    }
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        self.poll_flush(cx)
    }
}
impl AsyncWrite for Cursor<Vec<u8>> {
    fn poll_write(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        task::Poll::Ready(io::Write::write(&mut self.inner, buf))
    }
    fn poll_flush(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        task::Poll::Ready(io::Write::flush(&mut self.inner))
    }
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        self.poll_flush(cx)
    }
}
impl<T> AsyncSeek for Cursor<T>
where
    T: AsRef<[u8]> + Unpin,
{
    fn poll_seek(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        pos: io::SeekFrom,
    ) -> task::Poll<Result<u64, io::Error>> {
        task::Poll::Ready(io::Seek::seek(&mut self.inner, pos))
    }
}
