use super::*;
use std::sync;
/// [juniper::Executor]
pub struct Executor<'r, 'a, CtxT>
where
    CtxT: 'a,
{
    pub(crate) current_selection_set: Option<&'r [juniper::Selection<'a>]>,
    pub(crate) context: &'a CtxT,
    pub(crate) errors: &'r sync::RwLock<Vec<juniper::ExecutionError<juniper::DefaultScalarValue>>>,
}
impl<'a, CtxT> Executor<'_, 'a, CtxT> {
    /// [juniper::Executor::context]
    pub fn context(&self) -> &'a CtxT {
        self.context
    }
    /// [juniper::Executor::resolve]
    pub fn resolve<T>(&self, value: &T) -> Result<Value, juniper::FieldError>
    where
        T: GraphQLValue<Context = CtxT> + ?Sized,
    {
        value.resolve(self.current_selection_set, self)
    }
    /// [juniper::Executor::resolve_with_ctx]
    pub fn resolve_with_ctx<NewCtxT, T>(&self, value: &T) -> Result<Value, juniper::FieldError>
    where
        NewCtxT: juniper::FromContext<CtxT>,
        T: GraphQLValue<Context = NewCtxT> + ?Sized,
    {
        self.replaced_context(<NewCtxT as juniper::FromContext<CtxT>>::from(self.context))
            .resolve(value)
    }
    /// [juniper::Executor::resolve_into_value]
    pub fn resolve_into_value<T>(&self, value: &T) -> Value
    where
        T: GraphQLValue<Context = CtxT>,
    {
        self.resolve(value).unwrap_or_else(|e| {
            self.push_error(e);
            Value::Null
        })
    }
    /// [juniper::Executor::push_error]
    pub fn push_error(&self, error: juniper::FieldError) {
        self.errors
            .write()
            .unwrap()
            .push(juniper::ExecutionError::at_origin(error));
    }
    /// [juniper::Executor::replaced_context]
    pub fn replaced_context<'b, NewCtxT>(&'b self, ctx: &'b NewCtxT) -> Executor<'b, 'b, NewCtxT> {
        Executor {
            current_selection_set: self.current_selection_set,
            context: ctx,
            errors: self.errors,
        }
    }
}
