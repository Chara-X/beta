use std::io;
/// [io::Write]
pub trait Write {
    /// [io::Write::write]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    /// [io::Write::flush]
    fn flush(&mut self) -> io::Result<()>;
}
