use super::*;
use std::io;
/// [smol::io::copy]
pub async fn copy<R, W>(reader: R, writer: W) -> Result<u64, io::Error>
where
    R: AsyncRead,
    W: AsyncWrite,
{
    todo!()
}
