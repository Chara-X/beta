use super::*;
use std::marker;
/// [std::sync::Weak]
pub struct Weak<T: ?Sized> {
    _data: marker::PhantomData<T>,
}
impl<T: ?Sized> Weak<T> {
    /// [std::sync::Weak::new]
    pub const fn new() -> Weak<T> {
        todo!()
    }
    /// [std::sync::Weak::upgrade]
    pub fn upgrade(&self) -> Option<Arc<T>> {
        todo!()
    }
    /// [std::sync::Weak::strong_count]
    pub fn strong_count(&self) -> usize {
        todo!()
    }
    /// [std::sync::Weak::weak_count]
    pub fn weak_count(&self) -> usize {
        todo!()
    }
    /// [std::sync::Weak::ptr_eq]
    pub fn ptr_eq(&self, other: &Weak<T>) -> bool {
        todo!()
    }
}
