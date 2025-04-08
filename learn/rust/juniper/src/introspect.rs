use super::*;
/// [juniper::introspect]
pub fn introspect<QueryT, MutationT, SubscriptionT>(
    root_node: &RootNode<'_, QueryT, MutationT, SubscriptionT>,
    context: &QueryT::Context,
    format: juniper::IntrospectionFormat,
) -> Result<
    (
        Value,
        Vec<juniper::ExecutionError<juniper::DefaultScalarValue>>,
    ),
    juniper::GraphQLError,
>
where
    QueryT: GraphQLType,
    MutationT: GraphQLType<Context = QueryT::Context>,
    SubscriptionT: GraphQLType<Context = QueryT::Context>,
{
    todo!()
}
