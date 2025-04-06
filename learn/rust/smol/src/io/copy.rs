use super::*;
use pin_project_lite::pin_project;
use std::{io, pin, task};
/// [smol::io::copy]
pub async fn copy<R, W>(reader: R, writer: W) -> Result<u64, io::Error>
where
    R: AsyncRead,
    W: AsyncWrite,
{
    CopyFuture {
        reader: BufReader::new(reader),
        writer,
        amt: 0,
    }
    .await
}
pin_project! {
    struct CopyFuture<R, W> {
        #[pin]
        reader: R,
        #[pin]
        writer: W,
        amt: u64,
    }
}
impl<R, W> Future for CopyFuture<R, W>
where
    R: AsyncBufRead,
    W: AsyncWrite,
{
    type Output = io::Result<u64>;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let mut this = self.project();
        loop {
            let buf = task::ready!(this.reader.as_mut().poll_fill_buf(cx))?;
            if buf.is_empty() {
                task::ready!(this.writer.as_mut().poll_flush(cx))?;
                return task::Poll::Ready(Ok(*this.amt));
            }
            let n = task::ready!(this.writer.as_mut().poll_write(cx, buf))?;
            if n == 0 {
                return task::Poll::Ready(Err(io::ErrorKind::WriteZero.into()));
            }
            *this.amt += n as u64;
            this.reader.as_mut().consume(n);
        }
    }
}
