use std::ops;
use std::sync;
pub struct LazyLock<T, F = fn() -> T> {
    refer: sync::LazyLock<T, F>,
}
impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    pub const fn new(f: F) -> LazyLock<T, F> {
        LazyLock {
            refer: sync::LazyLock::new(f),
        }
    }
}
impl<T, F: FnOnce() -> T> ops::Deref for LazyLock<T, F> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.refer
    }
}
