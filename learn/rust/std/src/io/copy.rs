use super::*;
use std::io;
/// [fn@io::copy]
pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    let mut buf = [0u8; 8192];
    let mut len = 0;
    loop {
        let bytes_read = match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) => return Err(e),
        };
        writer.write_all(&buf[..bytes_read])?;
        len += bytes_read as u64;
    }
    Ok(len)
}
