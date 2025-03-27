use super::*;
use std::io;
/// [io::BufRead]
pub trait BufRead: Read {
    /// [io::BufRead::fill_buf]
    fn fill_buf(&mut self) -> io::Result<&[u8]>;
    /// [io::BufRead::consume]
    fn consume(&mut self, amt: usize);
}
