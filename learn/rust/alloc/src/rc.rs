extern crate alloc;
pub use alloc::rc;
pub use core::ops;
pub struct Rc<T: ?Sized> {
    refer: rc::Rc<T>,
}
impl<T> Rc<T> {
    pub fn new(value: T) -> Rc<T> {
        Rc {
            refer: rc::Rc::new(value),
        }
    }
}
impl<T: ?Sized> Rc<T> {
    pub fn strong_count(this: &Self) -> usize {
        rc::Rc::strong_count(&this.refer)
    }
    pub fn weak_count(this: &Self) -> usize {
        rc::Rc::weak_count(&this.refer)
    }
    pub fn downgrade(this: &Rc<T>) -> Weak<T> {
        Weak {
            refer: rc::Rc::downgrade(&this.refer),
        }
    }
}
impl<T: ?Sized> Clone for Rc<T> {
    fn clone(&self) -> Self {
        Rc {
            refer: self.refer.clone(),
        }
    }
}
impl<T: ?Sized> ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.refer
    }
}
pub struct Weak<T: ?Sized> {
    refer: rc::Weak<T>,
}
impl<T> Weak<T> {
    pub const fn new() -> Weak<T> {
        Weak {
            refer: rc::Weak::new(),
        }
    }
}
impl<T: ?Sized> Weak<T> {
    pub fn strong_count(this: &Self) -> usize {
        rc::Weak::strong_count(&this.refer)
    }
    pub fn weak_count(this: &Self) -> usize {
        rc::Weak::weak_count(&this.refer)
    }
    pub fn upgrade(&self) -> Option<Rc<T>> {
        self.refer.upgrade().map(|refer| Rc { refer })
    }
}
impl<T: ?Sized> Clone for Weak<T> {
    fn clone(&self) -> Self {
        Weak {
            refer: self.refer.clone(),
        }
    }
}
