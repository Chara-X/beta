use graphql_parser::{query, schema};
/// [schema::parse_schema]
pub fn parse_schema<'a, T>(s: &'a str) -> Result<Document<'a, T>, schema::ParseError>
where
    T: schema::Text<'a>,
{
    todo!()
}
/// [schema::Document]
pub struct Document<'a, T>
where
    T: schema::Text<'a>,
{
    /// [schema::Document::definitions]
    pub definitions: Vec<Definition<'a, T>>,
}
/// [schema::Definition]
pub enum Definition<'a, T: schema::Text<'a>> {
    /// [schema::Definition::SchemaDefinition]
    SchemaDefinition(SchemaDefinition<'a, T>),
    /// [schema::Definition::TypeDefinition]
    TypeDefinition(TypeDefinition<'a, T>),
}
/// [schema::SchemaDefinition]
pub struct SchemaDefinition<'a, T: schema::Text<'a>> {
    /// [schema::SchemaDefinition::query]
    pub query: Option<T::Value>,
    /// [schema::SchemaDefinition::mutation]
    pub mutation: Option<T::Value>,
    /// [schema::SchemaDefinition::subscription]
    pub subscription: Option<T::Value>,
}
/// [schema::TypeDefinition]
pub enum TypeDefinition<'a, T: schema::Text<'a>> {
    /// [schema::TypeDefinition::Object]
    Object(ObjectType<'a, T>),
    /// [schema::TypeDefinition::Scalar]
    Scalar(ScalarType<'a, T>),
}
/// [schema::ObjectType]
pub struct ObjectType<'a, T: schema::Text<'a>> {
    /// [schema::ObjectType::name]
    pub name: T::Value,
    /// [schema::ObjectType::description]
    pub description: Option<String>,
    /// [schema::ObjectType::fields]
    pub fields: Vec<Field<'a, T>>,
}
/// [schema::Field]
pub struct Field<'a, T: schema::Text<'a>> {
    /// [schema::Field::name]
    pub name: T::Value,
    /// [schema::Field::description]
    pub description: Option<String>,
    /// [schema::Field::arguments]
    pub arguments: Vec<schema::Value<'a, T>>,
    /// [schema::Field::field_type]
    pub field_type: query::Type<'a, T>,
}
/// [schema::ScalarType]
pub struct ScalarType<'a, T: schema::Text<'a>> {
    /// [schema::ScalarType::name]
    pub name: T::Value,
    /// [schema::ScalarType::description]
    pub description: Option<String>,
}
