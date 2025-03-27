use super::*;
use std::fmt;
use std::marker;
/// [fmt::Formatter]
pub struct Formatter<'a> {
    _data: marker::PhantomData<&'a ()>,
}
impl Formatter<'_> {
    /// [fmt::Formatter::fill]
    pub fn fill(&self) -> char {
        todo!()
    }
    /// [fmt::Formatter::align]
    pub fn align(&self) -> Option<fmt::Alignment> {
        todo!()
    }
    /// [fmt::Formatter::width]
    pub fn width(&self) -> Option<usize> {
        todo!()
    }
    /// [fmt::Formatter::precision]
    pub fn precision(&self) -> Option<usize> {
        todo!()
    }
    /// [fmt::Formatter::alternate]
    pub fn alternate(&self) -> bool {
        todo!()
    }
    /// [fmt::Formatter::write_str]
    pub fn write_str(&mut self, data: &str) -> Result<(), fmt::Error> {
        todo!()
    }
    /// [fmt::Formatter::write_fmt]
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) -> Result<(), fmt::Error> {
        todo!()
    }
    /// [fmt::Formatter::pad]
    pub fn pad(&mut self, s: &str) -> Result<(), fmt::Error> {
        todo!()
    }
}
