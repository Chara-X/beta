use super::*;
use juniper::meta;
/// [juniper::GraphQLType]
pub trait GraphQLType: GraphQLValue {
    /// [juniper::GraphQLType::name]
    fn name() -> Option<&'static str>;
    /// [juniper::GraphQLType::meta]
    fn meta<'r>(registry: &mut executor::Registry<'r>) -> meta::MetaType<'r>;
}
/// [juniper::GraphQLObject]
pub trait GraphQLObject: GraphQLType {}
/// [juniper::GraphQLInterface]
pub trait GraphQLInterface: GraphQLType {}
/// [juniper::GraphQLUnion]
pub trait GraphQLUnion: GraphQLType {}
