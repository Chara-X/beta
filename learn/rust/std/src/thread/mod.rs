//! [thread]
use std::{io, thread, time};
/// [std::thread::current]
pub fn current() -> Thread {
    todo!()
}
/// [std::thread::sleep]
pub fn sleep(dur: time::Duration) {
    todo!()
}
/// [thread::Builder]
pub struct Builder {}
impl Builder {
    /// [thread::Builder::new]
    pub fn new() -> Builder {
        todo!()
    }
    /// [thread::Builder::name]
    pub fn name(self, name: String) -> Builder {
        todo!()
    }
    /// [thread::Builder::stack_size]
    pub fn stack_size(self, size: usize) -> Builder {
        todo!()
    }
    /// [thread::Builder::spawn]
    pub fn spawn<F, T>(self, f: F) -> io::Result<thread::JoinHandle<T>> {
        todo!()
    }
}
/// [thread::Thread]
pub struct Thread();
impl Thread {
    /// [thread::Thread::id]
    pub fn id(&self) -> thread::ThreadId {
        todo!()
    }
    /// [thread::Thread::name]
    pub fn name(&self) -> Option<&str> {
        todo!()
    }
}
