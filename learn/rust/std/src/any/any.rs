#![allow(missing_docs)]
use super::*;
/// [std::any::Any]
pub trait Any: 'static {
    /// [std::any::Any::type_id]
    fn type_id(&self) -> TypeId;
}
impl dyn Any {
    pub fn is<T>(&self) -> bool
    where
        T: Any,
    {
        self.type_id() == TypeId::of::<T>()
    }
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Any as *const T)) }
        } else {
            None
        }
    }
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Any,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Any as *mut T)) }
        } else {
            None
        }
    }
}
impl<T> Any for T
where
    T: 'static + ?Sized,
{
    fn type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}
