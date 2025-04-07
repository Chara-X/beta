use super::*;
/// [juniper::ToInputValue]
pub trait ToInputValue<S>: Sized {
    /// [juniper::ToInputValue::to_input_value]
    fn to_input_value(&self) -> InputValue<S>;
}
