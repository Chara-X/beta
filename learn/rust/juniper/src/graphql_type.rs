use super::*;
use juniper::meta;
/// [juniper::GraphQLType]
pub trait GraphQLType: GraphQLValue {
    /// [juniper::GraphQLType::name]
    fn name(info: &Self::TypeInfo) -> Option<&str>;
    /// [juniper::GraphQLType::meta]
    fn meta<'r>(info: &Self::TypeInfo, registry: &mut Registry<'r>) -> meta::MetaType<'r>;
}
