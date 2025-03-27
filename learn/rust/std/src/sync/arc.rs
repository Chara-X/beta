use super::*;
use std::{any, marker, pin};
/// [std::sync::Arc]
pub struct Arc<T: ?Sized> {
    _data: marker::PhantomData<T>,
}
impl<T> Arc<T> {
    /// [std::sync::Arc::new]
    pub fn new(data: T) -> Arc<T> {
        todo!()
    }
    /// [std::sync::Arc::pin]
    pub fn pin(data: T) -> pin::Pin<Arc<T>> {
        todo!()
    }
    /// [std::sync::Arc::into_inner]
    pub fn into_inner(this: Arc<T>) -> Option<T> {
        todo!()
    }
}
impl<T: ?Sized> Arc<T> {
    /// [std::sync::Arc::strong_count]
    pub fn strong_count(this: &Arc<T>) -> usize {
        todo!()
    }
    /// [std::sync::Arc::weak_count]
    pub fn weak_count(this: &Arc<T>) -> usize {
        todo!()
    }
    /// [std::sync::Arc::downgrade]
    pub fn downgrade(this: &Arc<T>) -> Weak<T> {
        todo!()
    }
    /// [std::sync::Arc::get_mut]
    pub fn get_mut(this: &mut Arc<T>) -> Option<&mut T> {
        todo!()
    }
    /// [std::sync::Arc::ptr_eq]
    pub fn ptr_eq(this: &Arc<T>, other: &Arc<T>) -> bool {
        todo!()
    }
}
impl<T: Clone> Arc<T> {
    /// [std::sync::Arc::unwrap_or_clone]
    pub fn unwrap_or_clone(this: Arc<T>) -> T {
        todo!()
    }
}

impl Arc<dyn any::Any + Send + Sync> {
    /// [std::sync::Arc::downcast]
    pub fn downcast<T>(self) -> Result<Arc<T>, Arc<dyn any::Any + Send + Sync>>
    where
        T: any::Any + Send + Sync,
    {
        todo!()
    }
}
impl<T> Clone for Arc<T>
where
    T: ?Sized,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
