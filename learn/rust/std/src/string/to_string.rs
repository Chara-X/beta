use super::*;
use std::fmt;
/// [std::string::ToString]
pub trait ToString {
    /// [std::string::ToString::to_string]
    fn to_string(&self) -> String;
}
impl<T> ToString for T
where
    T: fmt::Display + ?Sized,
{
    fn to_string(&self) -> String {
        todo!()
    }
}
