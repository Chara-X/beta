use super::*;
use std::io;
/// [fn@io::copy]
pub fn copy<R, W>(reader: &mut R, writer: &mut W) -> io::Result<u64>
where
    R: Read + ?Sized,
    W: Write + ?Sized,
{
    todo!()
}
