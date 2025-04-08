use super::*;
use juniper::meta;
use std::collections;
/// [juniper::executor::Registry]
pub struct Registry<'r> {
    /// [juniper::executor::Registry::types]
    pub types: collections::HashMap<String, meta::MetaType<'r>>,
}
impl<'r> Registry<'r> {
    /// [juniper::executor::Registry::field]
    pub fn field<T>(&mut self, name: &str) -> meta::Field<'r, juniper::DefaultScalarValue>
    where
        T: GraphQLType + ?Sized,
    {
        todo!()
    }
    /// [juniper::Registry::build_object_type]
    pub fn build_object_type<T>(
        &mut self,
        fields: &[meta::Field<'r, juniper::DefaultScalarValue>],
    ) -> meta::ObjectMeta<'r, juniper::DefaultScalarValue>
    where
        T: GraphQLType + ?Sized,
    {
        todo!()
    }
}
