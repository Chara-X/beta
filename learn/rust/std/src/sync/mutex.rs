use std::marker;
use std::sync;
/// [sync::Mutex]
pub struct Mutex<T: ?Sized> {
    _mk: marker::PhantomData<T>,
}
impl<T> Mutex<T> {
    /// [sync::Mutex::new]
    pub const fn new(t: T) -> Mutex<T> {
        todo!()
    }
    /// [sync::Mutex::lock]
    pub fn lock(&self) -> sync::LockResult<sync::MutexGuard<'_, T>> {
        todo!()
    }
}
