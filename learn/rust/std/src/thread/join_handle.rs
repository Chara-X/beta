use super::*;
use std::marker;
use std::thread;
/// [thread::JoinHandle]
pub struct JoinHandle<T>(marker::PhantomData<T>);
impl<T> JoinHandle<T> {
    /// [thread::JoinHandle::thread]
    pub fn thread(&self) -> &Thread {
        todo!()
    }
    /// [thread::JoinHandle::join]
    pub fn join(self) -> thread::Result<T> {
        todo!()
    }
}
