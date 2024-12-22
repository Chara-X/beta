use std::sync;
pub struct Mutex<T: ?Sized> {
    refer: sync::Mutex<T>,
}

impl<T> Mutex<T> {
    pub const fn new(t: T) -> Mutex<T> {
        Mutex {
            refer: sync::Mutex::new(t),
        }
    }
    pub fn lock(&self) -> sync::LockResult<sync::MutexGuard<'_, T>> {
        self.refer.lock()
    }
}
