//! [io]
mod buffered;
mod stdio;
pub use self::buffered::*;
pub use self::stdio::*;
use std::io;
/// [io::Read]
pub trait Read {
    /// [io::Read::read]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}
/// [io::Write]
pub trait Write {
    /// [io::Write::write]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    /// [io::Write::flush]
    fn flush(&mut self) -> io::Result<()>;
}
/// [io::Seek]
pub trait Seek {
    /// [io::Seek::seek]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64>;
}
