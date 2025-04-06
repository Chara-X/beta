use crate::executor;
/// [juniper::GraphQLValue]
pub trait GraphQLValue {
    /// [juniper::GraphQLValue::Context]
    type Context;
    /// [juniper::GraphQLValue::TypeInfo]
    type TypeInfo;
    /// [juniper::GraphQLValue::type_name]
    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str>;
    /// [juniper::GraphQLValue::resolve]
    fn resolve(
        &self,
        info: &Self::TypeInfo,
        selection_set: Option<&[juniper::Selection<'_>]>,
        executor: &executor::Executor<'_, '_, Self::Context>,
    ) -> Result<juniper::Value, juniper::FieldError> {
        todo!()
    }
    /// [juniper::GraphQLValue::resolve_field]
    fn resolve_field(
        &self,
        _info: &Self::TypeInfo,
        _field_name: &str,
        _arguments: &juniper::Arguments<'_>,
        _executor: &executor::Executor<'_, '_, Self::Context>,
    ) -> Result<juniper::Value, juniper::FieldError> {
        todo!()
    }
}
