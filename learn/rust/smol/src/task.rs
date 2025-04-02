use std::{marker, pin, task};
/// [smol::Task]
pub struct Task<T, M = ()> {
    _data: marker::PhantomData<(T, M)>,
}
impl<T, M> Task<T, M> {
    /// [smol::Task::detach]
    pub fn detach(self) {
        todo!()
    }
    /// [smol::Task::cancel]
    pub async fn cancel(self) -> Option<T> {
        todo!()
    }
}
impl<T, M> Future for Task<T, M> {
    type Output = T;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        todo!()
    }
}
impl<T, M> Drop for Task<T, M> {
    fn drop(&mut self) {
        todo!()
    }
}
