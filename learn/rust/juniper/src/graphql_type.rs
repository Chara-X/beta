use super::*;
use juniper::meta;
/// [juniper::GraphQLType]
pub trait GraphQLType: GraphQLValue {
    /// [juniper::GraphQLType::name]
    fn name() -> Option<&'static str>;
    /// [juniper::GraphQLType::meta]
    fn meta<'r>(registry: &mut Registry<'r>) -> meta::MetaType<'r>;
}
/// [juniper::GraphQLValue]
pub trait GraphQLValue {
    /// [juniper::GraphQLValue::Context]
    type Context;
    /// [juniper::GraphQLValue::type_name]
    fn type_name<'i>(&self) -> Option<&'i str>;
    /// [juniper::GraphQLValue::concrete_type_name]
    fn concrete_type_name(&self, context: &Self::Context) -> String {
        todo!()
    }
    /// [juniper::GraphQLValue::resolve]
    fn resolve(
        &self,
        selection_set: Option<&[juniper::Selection<'_>]>,
        executor: &Executor<'_, '_, Self::Context>,
    ) -> Result<Value, juniper::FieldError> {
        todo!()
    }
    /// [juniper::GraphQLValue::resolve_field]
    fn resolve_field(
        &self,
        field_name: &str,
        arguments: &Arguments<'_>,
        executor: &Executor<'_, '_, Self::Context>,
    ) -> Result<Value, juniper::FieldError> {
        todo!()
    }
    /// [juniper::GraphQLValue::resolve_into_type]
    fn resolve_into_type(
        &self,
        type_name: &str,
        selection_set: Option<&[juniper::Selection<'_>]>,
        executor: &Executor<'_, '_, Self::Context>,
    ) -> Result<Value, juniper::FieldError> {
        todo!()
    }
}

use std::marker;
/// [juniper::Executor]
pub struct Executor<'r, 'a, CtxT>
where
    CtxT: 'a,
{
    _data: marker::PhantomData<(&'r (), &'a CtxT)>,
}
impl<'r, CtxT> Executor<'r, '_, CtxT> {
    /// [juniper::Executor::context]
    pub fn context(&self) -> &'r CtxT {
        todo!()
    }
    /// [juniper::Executor::resolve]
    pub fn resolve<T>(&self, value: &T) -> Result<Value, juniper::FieldError>
    where
        T: GraphQLValue<Context = CtxT> + ?Sized,
    {
        todo!()
    }
    /// [juniper::Executor::resolve_with_ctx]
    pub fn resolve_with_ctx<NewCtxT, T>(&self, value: &T) -> Result<Value, juniper::FieldError>
    where
        NewCtxT: juniper::FromContext<CtxT>,
        T: GraphQLValue<Context = NewCtxT> + ?Sized,
    {
        todo!()
    }
    /// [juniper::Executor::replaced_context]
    pub fn replaced_context<'b, NewCtxT>(&'b self, ctx: &'b NewCtxT) -> Executor<'b, 'b, NewCtxT> {
        todo!()
    }
}
/// [juniper::Arguments]
pub struct Arguments<'a> {
    _data: marker::PhantomData<&'a ()>,
}
impl Arguments<'_> {
    /// [juniper::Arguments::get]
    pub fn get<T>(&self, name: &str) -> Result<Option<T>, juniper::FieldError>
    where
        T: juniper::FromInputValue,
        T::Error: juniper::IntoFieldError,
    {
        todo!()
    }
}
