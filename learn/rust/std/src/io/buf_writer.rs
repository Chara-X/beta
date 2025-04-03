use super::*;
use std::{io, mem, ptr};
/// [io::BufWriter]
pub struct BufWriter<W: ?Sized + Write> {
    buf: Vec<u8>,
    inner: W,
}
impl<W: Write> BufWriter<W> {
    /// [io::BufWriter::new]
    pub fn new(inner: W) -> BufWriter<W> {
        BufWriter {
            buf: Vec::with_capacity(8192),
            inner,
        }
    }
    /// [io::BufWriter::into_inner]
    pub fn into_inner(self) -> Result<W, io::IntoInnerError<BufWriter<W>>> {
        Ok(unsafe { ptr::read(&mem::ManuallyDrop::new(self).inner) })
    }
}
impl<W: ?Sized + Write> BufWriter<W> {
    /// [io::BufWriter::buffer]
    pub fn buffer(&self) -> &[u8] {
        &self.buf
    }
    /// [io::BufWriter::get_ref]
    pub fn get_ref(&self) -> &W {
        &self.inner
    }
    /// [io::BufWriter::get_mut]
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }
    fn spare_capacity(&self) -> usize {
        self.buf.capacity() - self.buf.len()
    }
    fn flush_buf(&mut self) -> io::Result<()> {
        match self.inner.write(&self.buf) {
            Ok(n) => {
                self.buf.drain(..n);
            }
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
impl<W: ?Sized + Write> Write for BufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if buf.len() < self.spare_capacity() {
            self.buf.extend_from_slice(buf);
            Ok(buf.len())
        } else {
            if buf.len() > self.spare_capacity() {
                self.flush_buf()?;
            }
            if buf.len() >= self.buf.capacity() {
                self.get_mut().write(buf)
            } else {
                self.buf.extend_from_slice(buf);
                Ok(buf.len())
            }
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        self.flush_buf().and_then(|()| self.get_mut().flush())
    }
}
impl<W: ?Sized + Write + Seek> Seek for BufWriter<W> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        self.flush_buf()?;
        self.get_mut().seek(pos)
    }
}
impl<W: ?Sized + Write> Drop for BufWriter<W> {
    fn drop(&mut self) {
        self.flush_buf().unwrap();
    }
}
