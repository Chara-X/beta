//! [std::rc]
use std::marker;
pub use std::ops;
pub use std::rc;
/// [rc::Rc]
pub struct Rc<T: ?Sized> {
    _mk: marker::PhantomData<T>,
}
impl<T> Rc<T> {
    /// [rc::Rc::new]
    pub fn new(value: T) -> Rc<T> {
        todo!()
    }
    /// [rc::Rc::clone]
    pub fn strong_count(this: &Self) -> usize {
        todo!()
    }
    /// [rc::Rc::downgrade]
    pub fn weak_count(this: &Self) -> usize {
        todo!()
    }
    /// [rc::Rc::downgrade]
    pub fn downgrade(this: &Rc<T>) -> Weak<T> {
        todo!()
    }
}
/// [rc::Weak]
pub struct Weak<T: ?Sized> {
    _mk: marker::PhantomData<T>,
}
impl<T> Weak<T> {
    /// [rc::Weak::new]
    pub const fn new() -> Weak<T> {
        todo!()
    }
    /// [rc::Weak::strong_count]
    pub fn strong_count(this: &Self) -> usize {
        todo!()
    }
    /// [rc::Weak::weak_count]
    pub fn weak_count(this: &Self) -> usize {
        todo!()
    }
    /// [rc::Weak::upgrade]
    pub fn upgrade(&self) -> Option<Rc<T>> {
        todo!()
    }
}
