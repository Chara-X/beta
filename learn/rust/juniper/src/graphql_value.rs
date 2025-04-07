use crate::executor;
/// [juniper::GraphQLValue]
pub trait GraphQLValue {
    /// [juniper::GraphQLValue::Context]
    type Context;
    /// [juniper::GraphQLValue::type_name]
    fn type_name<'i>(&self) -> Option<&'i str>;
    /// [juniper::GraphQLValue::resolve]
    fn resolve(
        &self,
        selection_set: Option<&[juniper::Selection<'_>]>,
        executor: &executor::Executor<'_, '_, Self::Context>,
    ) -> Result<juniper::Value, juniper::FieldError> {
        todo!()
    }
    /// [juniper::GraphQLValue::resolve_field]
    fn resolve_field(
        &self,
        field_name: &str,
        arguments: &juniper::Arguments<'_>,
        executor: &executor::Executor<'_, '_, Self::Context>,
    ) -> Result<juniper::Value, juniper::FieldError> {
        todo!()
    }
}
