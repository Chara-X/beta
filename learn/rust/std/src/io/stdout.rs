use super::*;
use std::{io, marker};
/// [io::stdout]
pub fn stdout() -> Stdout {
    Stdout {}
}
/// [io::Stdout]
pub struct Stdout {}
impl Stdout {
    /// [io::Stdout::lock]
    pub fn lock(&self) -> StdoutLock<'static> {
        StdoutLock {
            _data: marker::PhantomData,
        }
    }
}
/// [io::StdoutLock]
pub struct StdoutLock<'a> {
    _data: marker::PhantomData<&'a ()>,
}
impl Write for StdoutLock<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }
    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}
