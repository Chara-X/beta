use core::any;
pub trait Any: any::Any {
    fn type_id(&self) -> any::TypeId;
}
impl dyn Any {
    pub fn is<T: Any>(&self) -> bool {
        (self as &dyn any::Any).is::<T>()
    }
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (self as &dyn any::Any).downcast_ref::<T>()
    }
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        (self as &mut dyn any::Any).downcast_mut::<T>()
    }
}
