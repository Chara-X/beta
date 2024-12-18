use core::any;
pub trait Any: 'static {
    fn type_id(&self) -> any::TypeId;
}
impl dyn Any {
    pub fn is<T: any::Any>(&self) -> bool {
        let x = self as &dyn any::Any;
        // let x = self as &dyn Any;
        <dyn any::Any>::is::<T>(x)
    }
}
