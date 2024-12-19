use core::any;
pub trait Any: 'static {
    fn type_id(&self) -> any::TypeId;
}
impl dyn Any {
    pub fn is<'a, T: Any>(&'a self) -> bool
    where
        &'a Self: any::Any,
    {
        <dyn any::Any>::is::<T>(&self)
    }
    pub fn downcast_ref<'a, T: Any>(&'a self) -> Option<&'a T>
    where
        &'a Self: any::Any,
    {
        let x = <dyn any::Any>::downcast_ref::<T>(&self);
        panic!()
    }
}
