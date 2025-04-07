/// [juniper::InputValue]
pub enum InputValue<S> {
    /// [juniper::InputValue::Null]
    Null,
    /// [juniper::InputValue::Scalar]
    Scalar(S),
    /// [juniper::InputValue::Enum]
    Enum(String),
    /// [juniper::InputValue::Variable]
    Variable(String),
    /// [juniper::InputValue::List]
    List(Vec<InputValue<S>>),
    /// [juniper::InputValue::Object]
    Object(Vec<(String, InputValue<S>)>),
}
