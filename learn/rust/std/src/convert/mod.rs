//! [std::convert]
/// [std::convert::From<T>]
pub trait From<T> {
    /// [std::convert::From::from]
    fn from(value: T) -> Self;
}
/// [std::convert::Into<T>]
pub trait Into<T> {
    /// [std::convert::Into::into]
    fn into(self) -> T;
}
