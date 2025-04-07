use std::borrow;
/// [juniper::Type]
pub enum Type<'a> {
    /// [juniper::Type::Named]
    Named(borrow::Cow<'a, str>),
    /// [juniper::Type::List]
    List(Box<Type<'a>>, Option<usize>),
    /// [juniper::Type::NonNullNamed]
    NonNullNamed(borrow::Cow<'a, str>),
    /// [juniper::Type::NonNullList]
    NonNullList(Box<Type<'a>>, Option<usize>),
}
impl Type<'_> {
    /// [juniper::Type::name]
    pub fn name(&self) -> Option<&str> {
        todo!()
    }
    /// [juniper::Type::innermost_name]
    pub fn innermost_name(&self) -> &str {
        todo!()
    }
    /// [juniper::Type::is_non_null]
    pub fn is_non_null(&self) -> bool {
        todo!()
    }
}
