use std::marker;
use tokio::task;
/// [task::JoinHandle]
pub struct JoinHandle<T> {
    _data: marker::PhantomData<T>,
}
impl<T> JoinHandle<T> {
    /// [task::JoinHandle::id]
    pub fn id(&self) -> task::Id {
        todo!()
    }
    /// [task::JoinHandle::abort]
    pub fn abort(&self) {
        todo!()
    }
}
impl<T> Future for JoinHandle<T> {
    type Output = Result<T, task::JoinError>;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        todo!()
    }
}
