use super::*;
use std::{io, pin};
/// [smol::io::BlockOn]
pub struct BlockOn<T>(T);
impl<T> BlockOn<T> {
    /// [smol::io::BlockOn::new]
    pub fn new(io: T) -> BlockOn<T> {
        BlockOn(io)
    }
    /// [smol::io::BlockOn::get_ref]
    pub fn get_ref(&self) -> &T {
        &self.0
    }
    /// [smol::io::BlockOn::get_mut]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }
    /// [smol::io::BlockOn::into_inner]
    pub fn into_inner(self) -> T {
        self.0
    }
}
impl<T> io::BufRead for BlockOn<T>
where
    T: AsyncBufRead + Unpin,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        crate::block_on(self.0.fill_buf())
    }
    fn consume(&mut self, amt: usize) {
        pin::Pin::new(&mut self.0).consume(amt)
    }
}
impl<T> io::Read for BlockOn<T>
where
    T: AsyncRead + Unpin,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        crate::block_on(self.0.read(buf))
    }
}
impl<T> io::Write for BlockOn<T>
where
    T: AsyncWrite + Unpin,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        crate::block_on(self.0.write(buf))
    }
    fn flush(&mut self) -> io::Result<()> {
        crate::block_on(self.0.flush())
    }
}
impl<T> io::Seek for BlockOn<T>
where
    T: AsyncSeek + Unpin,
{
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        crate::block_on(self.0.seek(pos))
    }
}
