use std::marker;
use std::thread;
/// [thread::ScopedJoinHandle]
pub struct ScopedJoinHandle<'scope, T>(marker::PhantomData<(&'scope (), T)>);
impl<T> ScopedJoinHandle<'_, T> {
    /// [thread::ScopedJoinHandle::thread]
    pub fn thread(&self) -> &thread::Thread {
        todo!()
    }
    /// [thread::ScopedJoinHandle::join]
    pub fn join(self) -> T {
        todo!()
    }
}
