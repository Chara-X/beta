use super::*;
use std::collections;
/// [juniper::execute_sync]
pub fn execute_sync<'a, QueryT, MutationT, SubscriptionT>(
    document_source: &'a str,
    operation_name: Option<&str>,
    root_node: &'a RootNode<'_, QueryT, MutationT, SubscriptionT>,
    variables: &collections::HashMap<String, juniper::InputValue>,
    context: &QueryT::Context,
) -> Result<
    (
        juniper::Value,
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
