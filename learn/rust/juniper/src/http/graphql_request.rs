use super::*;
/// [juniper::http::GraphQLRequest]
pub struct GraphQLRequest {
    /// [juniper::http::GraphQLRequest::query]
    pub query: String,
    /// [juniper::http::GraphQLRequest::operation_name]
    pub operation_name: Option<String>,
    /// [juniper::http::GraphQLRequest::variables]
    pub variables: Option<juniper::InputValue>,
}
