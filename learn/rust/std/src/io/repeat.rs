use super::*;
use std::io;
/// [io::repeat]
pub const fn repeat(byte: u8) -> Repeat {
    Repeat { byte }
}
/// [io::Repeat]
pub struct Repeat {
    byte: u8,
}
impl Read for Repeat {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        for slot in &mut *buf {
            *slot = self.byte;
        }
        Ok(buf.len())
    }
}
