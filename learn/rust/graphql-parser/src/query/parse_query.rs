use graphql_parser::query;
/// [query::parse_query]
pub fn parse_query<'a, S>(s: &'a str) -> Result<Document<'a, S>, query::ParseError>
where
    S: query::Text<'a>,
{
    todo!()
}
/// [query::Document]
pub struct Document<'a, T: query::Text<'a>> {
    /// [query::Document::definitions]
    pub definitions: Vec<Definition<'a, T>>,
}
/// [query::Definition]
pub enum Definition<'a, T: query::Text<'a>> {
    /// [query::Definition::Operation]
    Operation(OperationDefinition<'a, T>),
    /// [query::Definition::Fragment]
    Fragment(query::FragmentDefinition<'a, T>),
}
/// [query::OperationDefinition]
pub enum OperationDefinition<'a, T: query::Text<'a>> {
    /// [query::OperationDefinition::SelectionSet]
    SelectionSet(SelectionSet<'a, T>),
    /// [query::OperationDefinition::Query]
    Query(query::Query<'a, T>),
    /// [query::OperationDefinition::Mutation]
    Mutation(query::Mutation<'a, T>),
    /// [query::OperationDefinition::Subscription]
    Subscription(query::Subscription<'a, T>),
}
/// [query::SelectionSet]
pub struct SelectionSet<'a, T: query::Text<'a>> {
    /// [query::SelectionSet::items]
    pub items: Vec<Selection<'a, T>>,
}
/// [query::Selection]
pub enum Selection<'a, T: query::Text<'a>> {
    /// [query::Selection::Field]
    Field(Field<'a, T>),
    /// [query::Selection::FragmentSpread]
    FragmentSpread(query::FragmentSpread<'a, T>),
    /// [query::Selection::InlineFragment]
    InlineFragment(query::InlineFragment<'a, T>),
}
/// [query::Field]
pub struct Field<'a, T: query::Text<'a>> {
    /// [query::Field::name]
    pub name: T::Value,
    /// [query::Field::arguments]
    pub arguments: Vec<(T::Value, query::Value<'a, T>)>,
    /// [query::Field::selection_set]
    pub selection_set: SelectionSet<'a, T>,
}
