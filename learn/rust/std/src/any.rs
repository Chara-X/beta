//! [std::any]
use std::any;
/// [any::Any]
pub trait Any {
    /// [any::Any::type_id]
    fn type_id(&self) -> any::TypeId;
}
impl dyn Any {
    ///
    pub fn is<T: Any>(&self) -> bool {
        todo!()
    }
    ///
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        todo!()
    }
    ///
    pub fn downcast_mut<T: Any>(&mut self) -> Option<&mut T> {
        todo!()
    }
}
