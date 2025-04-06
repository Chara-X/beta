use super::*;
use std::marker;
/// [smol::io::BufReader]
pub struct BufReader<R> {
    _data: marker::PhantomData<R>,
}
impl<R> BufReader<R>
where
    R: AsyncRead,
{
    /// [smol::io::BufReader::new]
    pub fn new(inner: R) -> BufReader<R> {
        todo!()
    }
}
impl<R> AsyncBufRead for BufReader<R>
where
    R: AsyncRead,
{
    fn poll_fill_buf(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<&[u8], std::io::Error>> {
        todo!()
    }
    fn consume(self: std::pin::Pin<&mut Self>, amt: usize) {
        todo!()
    }
}
impl<R> AsyncRead for BufReader<R>
where
    R: AsyncRead,
{
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        todo!()
    }
}
