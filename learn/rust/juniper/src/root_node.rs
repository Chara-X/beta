use super::*;
use std::marker;
/// [juniper::RootNode]
pub struct RootNode<'a, QueryT: GraphQLType, MutationT: GraphQLType, SubscriptionT: GraphQLType> {
    _data: marker::PhantomData<(&'a (), QueryT, MutationT, SubscriptionT)>,
}
impl<'a, QueryT, MutationT, SubscriptionT> RootNode<'a, QueryT, MutationT, SubscriptionT>
where
    QueryT: GraphQLType<TypeInfo = ()>,
    MutationT: GraphQLType<TypeInfo = ()>,
    SubscriptionT: GraphQLType<TypeInfo = ()>,
{
    /// [juniper::RootNode::new]
    pub fn new(query: QueryT, mutation: MutationT, subscription: SubscriptionT) -> Self {
        todo!()
    }
}
impl<'a, QueryT, MutationT, SubscriptionT> RootNode<'a, QueryT, MutationT, SubscriptionT>
where
    QueryT: GraphQLType,
    MutationT: GraphQLType,
    SubscriptionT: GraphQLType,
{
    /// [juniper::RootNode::as_sdl]
    pub fn as_sdl(&self) -> String {
        todo!()
    }
}
impl<'a, QueryT, MutationT, SubscriptionT> GraphQLValue
    for RootNode<'a, QueryT, MutationT, SubscriptionT>
where
    QueryT: GraphQLType,
    MutationT: GraphQLType<Context = QueryT::Context>,
    SubscriptionT: GraphQLType<Context = QueryT::Context>,
{
    type Context = <QueryT as GraphQLValue>::Context;
    type TypeInfo = <QueryT as GraphQLValue>::TypeInfo;
    fn type_name<'i>(&self, info: &'i Self::TypeInfo) -> Option<&'i str> {
        todo!()
    }
    fn resolve(
        &self,
        info: &Self::TypeInfo,
        selection_set: Option<&[juniper::Selection<'_>]>,
        executor: &executor::Executor<'_, '_, Self::Context>,
    ) -> Result<juniper::Value, juniper::FieldError> {
        todo!()
    }
}
