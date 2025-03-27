/// [std::any::TypeId]
pub struct TypeId {}
impl TypeId {
    /// [std::any::TypeId::of]
    pub fn of<T>() -> TypeId
    where
        T: 'static + ?Sized,
    {
        todo!()
    }
}
impl PartialEq for TypeId {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
