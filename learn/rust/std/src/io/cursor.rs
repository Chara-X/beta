use super::*;
use std::io;
/// [io::Cursor]
pub struct Cursor<T> {
    inner: T,
    pos: u64,
}
impl<T> Cursor<T> {
    /// [io::Cursor::new]
    pub const fn new(inner: T) -> Cursor<T> {
        Cursor { inner, pos: 0 }
    }
    /// [io::Cursor::position]
    pub const fn position(&self) -> u64 {
        self.pos
    }
    /// [io::Cursor::set_position]
    pub fn set_position(&mut self, pos: u64) {
        self.pos = pos;
    }
    /// [io::Cursor::get_ref]
    pub const fn get_ref(&self) -> &T {
        &self.inner
    }
    /// [io::Cursor::get_mut]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }
    /// [io::Cursor::into_inner]
    pub fn into_inner(self) -> T {
        self.inner
    }
}
impl<T> BufRead for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        Ok(&self.inner.as_ref()[self.pos as usize..])
    }
    fn consume(&mut self, amt: usize) {
        self.pos += amt as u64;
    }
}
impl<T> Read for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = (&self.inner.as_ref()[self.pos as usize..]).read(buf)?;
        self.pos += n as u64;
        Ok(n)
    }
}
impl<T> Seek for Cursor<T>
where
    T: AsRef<[u8]>,
{
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.pos = match pos {
            io::SeekFrom::Start(n) => n,
            io::SeekFrom::End(n) => self.inner.as_ref().len() as u64 + n as u64,
            io::SeekFrom::Current(n) => self.pos + n as u64,
        };
        Ok(self.pos)
    }
}
impl Write for Cursor<&mut [u8]> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = (&mut self.inner[self.pos as usize..]).write(buf)?;
        self.pos += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Write for Cursor<&mut Vec<u8>> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.resize(self.pos as usize + buf.len(), 0);
        let n = (&mut self.inner[self.pos as usize..]).write(buf)?;
        self.pos += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Write for Cursor<Vec<u8>> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.resize(self.pos as usize + buf.len(), 0);
        let n = (&mut self.inner[self.pos as usize..]).write(buf)?;
        self.pos += n as u64;
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
