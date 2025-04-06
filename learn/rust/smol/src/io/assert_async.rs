use super::*;
use std::{io, pin, task};
/// [smol::io::AssertAsync]
pub struct AssertAsync<T>(T);
impl<T> AssertAsync<T> {
    /// [smol::io::AssertAsync::new]
    pub fn new(io: T) -> AssertAsync<T> {
        AssertAsync(io)
    }
    /// [smol::io::AssertAsync::get_ref]
    pub fn get_ref(&self) -> &T {
        &self.0
    }
    /// [smol::io::AssertAsync::get_mut]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// [smol::io::AssertAsync::into_inner]
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<T> AsyncRead for AssertAsync<T>
where
    T: io::Read,
{
    fn poll_read(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        task::Poll::Ready(self.0.read(buf))
    }
}
impl<T> AsyncWrite for AssertAsync<T>
where
    T: io::Write,
{
    fn poll_write(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        task::Poll::Ready(self.0.write(buf))
    }
    fn poll_flush(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        task::Poll::Ready(self.0.flush())
    }
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), io::Error>> {
        self.poll_flush(cx)
    }
}
impl<T> AsyncSeek for AssertAsync<T>
where
    T: io::Seek,
{
    fn poll_seek(
        mut self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        pos: io::SeekFrom,
    ) -> task::Poll<Result<u64, io::Error>> {
        task::Poll::Ready(self.0.seek(pos))
    }
}
impl<T> Unpin for AssertAsync<T> {}
