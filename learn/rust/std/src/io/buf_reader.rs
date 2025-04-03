use super::*;
use std::{
    cmp,
    io::{self, Read as _},
};
/// [io::BufReader]
pub struct BufReader<R: ?Sized> {
    buf: Vec<u8>,
    pos: usize,
    filled: usize,
    inner: R,
}
impl<R: io::Read> BufReader<R> {
    /// [io::BufReader::new]
    pub fn new(inner: R) -> BufReader<R> {
        BufReader {
            buf: vec![0; 8192],
            pos: 0,
            filled: 0,
            inner,
        }
    }
}
impl<R: ?Sized> BufReader<R> {
    /// [io::BufReader::buffer]
    pub fn buffer(&self) -> &[u8] {
        &self.buf[self.pos..self.filled]
    }
    /// [io::BufReader::get_ref]
    pub fn get_ref(&self) -> &R {
        &self.inner
    }
    /// [io::BufReader::get_mut]
    pub fn get_mut(&mut self) -> &mut R {
        &mut self.inner
    }
    /// [io::BufReader::into_inner]
    pub fn into_inner(self) -> R
    where
        R: Sized,
    {
        self.inner
    }
    fn discard_buffer(&mut self) {
        self.pos = 0;
        self.filled = 0;
    }
}
impl<R: ?Sized + Read> BufRead for BufReader<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.pos >= self.filled {
            self.pos = 0;
            self.filled = self.inner.read(&mut self.buf)?;
        }
        Ok(self.buffer())
    }
    fn consume(&mut self, amt: usize) {
        self.pos = cmp::min(self.pos + amt, self.filled);
    }
}
impl<R: ?Sized + Read> Read for BufReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.pos == self.filled && buf.len() >= self.buf.len() {
            self.discard_buffer();
            return self.inner.read(buf);
        }
        let n = self.fill_buf()?.read(buf)?;
        self.consume(n);
        Ok(n)
    }
}
impl<R: ?Sized + Seek> Seek for BufReader<R> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let res: u64 = if let io::SeekFrom::Current(n) = pos {
            self.inner
                .seek(io::SeekFrom::Current(n - (self.filled - self.pos) as i64))?
        } else {
            self.inner.seek(pos)?
        };
        self.discard_buffer();
        Ok(res)
    }
}
