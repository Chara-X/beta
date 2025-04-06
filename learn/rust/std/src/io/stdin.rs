use super::*;
use std::io;
use std::marker;
/// [io::stdin]
pub fn stdin() -> Stdin {
    Stdin {}
}
/// [io::Stdin]
pub struct Stdin {}
impl Stdin {
    /// [io::Stdin::lock]
    pub fn lock(&self) -> StdinLock<'static> {
        StdinLock {
            _data: marker::PhantomData,
        }
    }
}
/// [io::StdinLock]
pub struct StdinLock<'a> {
    _data: marker::PhantomData<&'a ()>,
}
impl BufRead for StdinLock<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        todo!()
    }
    fn consume(&mut self, amt: usize) {
        todo!()
    }
}
impl Read for StdinLock<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}
