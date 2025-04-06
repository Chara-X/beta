use std::{cmp, io};
/// [io::Read]
pub trait Read {
    /// [io::Read::read]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}
impl<R: Read + ?Sized> Read for &mut R {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read(buf)
    }
}
impl Read for &[u8] {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let amt = cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);
        buf[..amt].copy_from_slice(a);
        *self = b;
        Ok(amt)
    }
}
