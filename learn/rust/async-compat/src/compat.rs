use std::{marker, pin, task};
/// [async_compat::Compat]
pub struct Compat<T> {
    _data: marker::PhantomData<T>,
}
impl<T> Compat<T> {
    /// [async_compat::Compat::new]
    pub fn new(t: T) -> Compat<T> {
        todo!()
    }
    /// [async_compat::Compat::get_ref]
    pub fn get_ref(&self) -> &T {
        todo!()
    }
    /// [async_compat::Compat::get_mut]
    pub fn get_mut(&mut self) -> &mut T {
        todo!()
    }
    /// [async_compat::Compat::into_inner]
    pub fn into_inner(self) -> T {
        todo!()
    }
}
impl<T: Future> Future for Compat<T> {
    type Output = <T as Future>::Output;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        todo!()
    }
}
impl<T: futures::io::AsyncRead> futures::io::AsyncRead for Compat<T> {
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut [u8],
    ) -> task::Poll<std::io::Result<usize>> {
        todo!()
    }
}
impl<T: tokio::io::AsyncRead> tokio::io::AsyncRead for Compat<T> {
    fn poll_read(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> task::Poll<std::io::Result<()>> {
        todo!()
    }
}
