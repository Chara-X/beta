use super::*;
use std::{io, marker};
/// [io::BufWriter]
pub struct BufWriter<W: ?Sized + Write> {
     // The buffer. Avoid using this like a normal `Vec` in common code paths.
    // That is, don't use `buf.push`, `buf.extend_from_slice`, or any other
    // methods that require bounds checking or the like. This makes an enormous
    // difference to performance (we may want to stop using a `Vec` entirely).
    buf: Vec<u8>,
    inner: W,
}
impl<W: Write> BufWriter<W> {
    /// [io::BufWriter::new]
    pub fn new(inner: W) -> BufWriter<W> {
        
    }
    /// [io::BufWriter::into_inner]
    pub fn into_inner(self) -> Result<W, io::IntoInnerError<BufWriter<W>>> {
        todo!()
    }
}
impl<W: ?Sized + Write> BufWriter<W> {
    /// [io::BufWriter::buffer]
    pub fn buffer(&self) -> &[u8] {
        todo!()
    }
    /// [io::BufWriter::get_ref]
    pub fn get_ref(&self) -> &W {
        todo!()
    }
    /// [io::BufWriter::get_mut]
    pub fn get_mut(&mut self) -> &mut W {
        todo!()
    }
}
impl<W: ?Sized + Write> Write for BufWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }
    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}
impl<W: ?Sized + Write + Seek> Seek for BufWriter<W> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        todo!()
    }
}
impl<W: ?Sized + Write> Drop for BufWriter<W> {
    fn drop(&mut self) {
        todo!()
    }
}
