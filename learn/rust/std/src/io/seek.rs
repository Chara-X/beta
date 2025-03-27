use std::io;
/// [io::Seek]
pub trait Seek {
    /// [io::Seek::seek]
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64>;
}
