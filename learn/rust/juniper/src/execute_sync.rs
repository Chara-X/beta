use super::*;
use graphql_parser::query;
use std::{collections, sync};
/// [juniper::execute_sync]
pub fn execute_sync<'a, QueryT, MutationT, SubscriptionT>(
    document_source: &'a str,
    operation_name: Option<&str>,
    root_node: &'a RootNode<'_, QueryT, MutationT, SubscriptionT>,
    variables: &collections::HashMap<String, Value>,
    context: &QueryT::Context,
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
    let document = query::parse_query::<String>(document_source).unwrap();
    let operation = document
        .definitions
        .iter()
        .find_map(|def| match def {
            query::Definition::Operation(op) => match op {
                query::OperationDefinition::Query(query) => {
                    if operation_name.is_none() || operation_name == query.name.as_deref() {
                        Some(op)
                    } else {
                        None
                    }
                }
                query::OperationDefinition::Mutation(mutation) => {
                    if operation_name.is_none() || operation_name == mutation.name.as_deref() {
                        Some(op)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        })
        .unwrap();
    let errors = sync::RwLock::new(Vec::new());
    Ok((
        Executor {
            current_selection_set: Some(&match operation {
                query::OperationDefinition::Query(query) => query
                    .selection_set
                    .items
                    .iter()
                    .map(from)
                    .collect::<Vec<_>>(),
                query::OperationDefinition::Mutation(mutation) => mutation
                    .selection_set
                    .items
                    .iter()
                    .map(from)
                    .collect::<Vec<_>>(),
                _ => unreachable!(),
            }),
            context,
            errors: &errors,
        }
        .resolve_into_value(root_node),
        errors.into_inner().unwrap(),
    ))
}
fn from<'a>(v: &'a query::Selection<'a, String>) -> juniper::Selection<'a> {
    todo!()
}
