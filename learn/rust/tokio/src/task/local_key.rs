use std::marker;
use tokio::task::futures;
/// [tokio::task::LocalKey]
pub struct LocalKey<T: 'static> {
    _data: marker::PhantomData<T>,
}
impl<T: 'static> LocalKey<T> {
    /// [tokio::task::LocalKey::scope]
    pub fn scope<F>(&'static self, value: T, f: F) -> futures::TaskLocalFuture<T, F>
    where
        F: Future,
    {
        todo!()
    }
    /// [tokio::task::LocalKey::with]
    pub fn with<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        todo!()
    }
}
impl<T: Clone + 'static> LocalKey<T> {
    /// [tokio::task::LocalKey::get]
    pub fn get(&'static self) -> T {
        todo!()
    }
}
