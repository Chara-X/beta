use super::*;
use std::marker;
/// [juniper::RootNode]
pub struct RootNode<'a, QueryT: GraphQLType, MutationT: GraphQLType, SubscriptionT: GraphQLType> {
    _data: marker::PhantomData<(&'a (), QueryT, MutationT, SubscriptionT)>,
}
impl<QueryT, MutationT, SubscriptionT> RootNode<'_, QueryT, MutationT, SubscriptionT>
where
    QueryT: GraphQLType,
    MutationT: GraphQLType,
    SubscriptionT: GraphQLType,
{
    /// [juniper::RootNode::new]
    pub fn new(query: QueryT, mutation: MutationT, subscription: SubscriptionT) -> Self {
        todo!()
    }
}
impl<QueryT, MutationT, SubscriptionT> RootNode<'_, QueryT, MutationT, SubscriptionT>
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
impl<QueryT, MutationT, SubscriptionT> GraphQLValue
    for RootNode<'_, QueryT, MutationT, SubscriptionT>
where
    QueryT: GraphQLType,
    MutationT: GraphQLType<Context = QueryT::Context>,
    SubscriptionT: GraphQLType<Context = QueryT::Context>,
{
    type Context = <QueryT as GraphQLValue>::Context;
    fn type_name<'i>(&self) -> Option<&'i str> {
        todo!()
    }
    fn resolve(
        &self,
        selection_set: Option<&[juniper::Selection<'_>]>,
        executor: &executor::Executor<'_, '_, Self::Context>,
    ) -> Result<juniper::Value, juniper::FieldError> {
        todo!()
    }
}
