use std::io;
/// [io::Read]
pub trait Read {
    /// [io::Read::read]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;
}
