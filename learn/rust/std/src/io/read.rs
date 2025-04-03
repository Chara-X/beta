use std::io;
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
