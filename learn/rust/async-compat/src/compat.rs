use pin_project_lite::pin_project;
use std::{pin, sync, task};
use tokio::runtime;
static TOKIO: sync::LazyLock<runtime::Handle> = sync::LazyLock::new(|| {
    runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .handle()
        .clone()
});
pin_project! {
    /// [async_compat::Compat]
    pub struct Compat<T> {
        #[pin]
        inner: Option<T>,
    }
}
impl<T> Compat<T> {
    /// [async_compat::Compat::new]
    pub fn new(t: T) -> Compat<T> {
        Compat { inner: Some(t) }
    }
    /// [async_compat::Compat::get_ref]
    pub fn get_ref(&self) -> &T {
        self.inner.as_ref().unwrap()
    }
    /// [async_compat::Compat::get_mut]
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.as_mut().unwrap()
    }
    /// [async_compat::Compat::into_inner]
    pub fn into_inner(mut self) -> T {
        self.inner.take().unwrap()
    }
}
impl<T: Future> Future for Compat<T> {
    type Output = <T as Future>::Output;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        let _guard = TOKIO.enter();
        self.project().inner.as_pin_mut().unwrap().poll(cx)
    }
}
impl<T: tokio::io::AsyncBufRead> futures::io::AsyncBufRead for Compat<T> {
    fn poll_fill_buf(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<&[u8]>> {
        todo!()
    }
    fn consume(self: pin::Pin<&mut Self>, amt: usize) {
        todo!()
    }
}
impl<T: futures::io::AsyncBufRead> tokio::io::AsyncBufRead for Compat<T> {
    fn poll_fill_buf(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<&[u8]>> {
        todo!()
    }
    fn consume(self: pin::Pin<&mut Self>, amt: usize) {
        todo!()
    }
}
impl<T: tokio::io::AsyncRead> futures::io::AsyncRead for Compat<T> {
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<std::io::Result<usize>> {
        todo!()
    }
}
impl<T: futures::io::AsyncRead> tokio::io::AsyncRead for Compat<T> {
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        todo!()
    }
}
impl<T: tokio::io::AsyncWrite> futures::io::AsyncWrite for Compat<T> {
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<std::io::Result<usize>> {
        todo!()
    }
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        todo!()
    }
    fn poll_close(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        todo!()
    }
}
impl<T: futures::io::AsyncWrite> tokio::io::AsyncWrite for Compat<T> {
    fn poll_write(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &[u8],
    ) -> task::Poll<Result<usize, std::io::Error>> {
        todo!()
    }
    fn poll_flush(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        todo!()
    }
    fn poll_shutdown(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Result<(), std::io::Error>> {
        todo!()
    }
}
impl<T: tokio::io::AsyncSeek> futures::io::AsyncSeek for Compat<T> {
    fn poll_seek(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        pos: std::io::SeekFrom,
    ) -> task::Poll<std::io::Result<u64>> {
        todo!()
    }
}
impl<T: futures::io::AsyncSeek> tokio::io::AsyncSeek for Compat<T> {
    fn start_seek(self: pin::Pin<&mut Self>, position: std::io::SeekFrom) -> std::io::Result<()> {
        todo!()
    }
    fn poll_complete(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<std::io::Result<u64>> {
        todo!()
    }
}
