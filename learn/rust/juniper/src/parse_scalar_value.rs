use juniper::parser;
/// [juniper::ParseScalarValue]
pub trait ParseScalarValue<S> {
    /// [juniper::ParseScalarValue::from_str]
    fn from_str(value: parser::ScalarToken<'_>) -> Result<S, juniper::ParseError>;
}
