use std::cell;
use std::marker;
/// [std::thread::LocalKey]
pub struct LocalKey<T: 'static> {
    _data: marker::PhantomData<T>,
}
impl<T: 'static> LocalKey<T> {
    /// [std::thread::LocalKey::with]
    pub fn with<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        todo!()
    }
}
impl<T: 'static> LocalKey<cell::Cell<T>> {
    /// [std::thread::LocalKey::replace]
    pub fn replace(&'static self, value: T) -> T {
        todo!()
    }
    /// [std::thread::LocalKey::get]
    pub fn get(&'static self) -> T
    where
        T: Copy,
    {
        todo!()
    }
    /// [std::thread::LocalKey::take]
    pub fn take(&'static self) -> T
    where
        T: Default,
    {
        todo!()
    }
}
impl<T: 'static> LocalKey<cell::RefCell<T>> {
    /// [std::thread::LocalKey::replace]
    pub fn replace(&'static self, value: T) -> T {
        todo!()
    }
    /// [std::thread::LocalKey::with_borrow]
    pub fn with_borrow<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        todo!()
    }
    /// [std::thread::LocalKey::with_borrow_mut]
    pub fn with_borrow_mut<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        todo!()
    }
    /// [std::thread::LocalKey::take]
    pub fn take(&'static self) -> T
    where
        T: Default,
    {
        todo!()
    }
}
