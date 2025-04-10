use super::*;
use std::collections;
const INTROSPECTION_QUERY: &str = include_str!("./query.graphql");
const INTROSPECTION_QUERY_WITHOUT_DESCRIPTIONS: &str =
    include_str!("./query_without_descriptions.graphql");
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
    execute_sync(
        match format {
            juniper::IntrospectionFormat::All => INTROSPECTION_QUERY,
            juniper::IntrospectionFormat::WithoutDescriptions => {
                INTROSPECTION_QUERY_WITHOUT_DESCRIPTIONS
            }
        },
        None,
        root_node,
        &collections::HashMap::new(),
        context,
    )
}
