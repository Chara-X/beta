use std::{cmp, io, mem};
/// [io::Write]
pub trait Write {
    /// [io::Write::write]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize>;
    /// [io::Write::flush]
    fn flush(&mut self) -> io::Result<()>;
    /// [io::Write::write_all]
    fn write_all(&mut self, mut buf: &[u8]) -> io::Result<()> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(io::ErrorKind::WriteZero.into());
                }
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}
impl Write for &mut [u8] {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let amt = cmp::min(buf.len(), self.len());
        let (a, b) = mem::take(self).split_at_mut(amt);
        a.copy_from_slice(&buf[..amt]);
        *self = b;
        Ok(amt)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
