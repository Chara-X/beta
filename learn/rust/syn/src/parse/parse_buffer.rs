use crate::buffer;
use std::{fmt, marker};
use syn::parse;
/// [parse::ParseBuffer]
pub struct ParseBuffer<'a> {
    _data: marker::PhantomData<&'a ()>,
}
impl<'a> ParseBuffer<'a> {
    /// [parse::ParseBuffer::cursor]
    pub fn cursor(&self) -> buffer::Cursor<'a> {
        todo!()
    }
    /// [parse::ParseBuffer::peek]
    pub fn peek<T: parse::Peek>(&self, token: T) -> bool {
        todo!()
    }
    /// [parse::ParseBuffer::step]
    pub fn step<F, R>(&self, function: F) -> syn::Result<R>
    where
        F: for<'c> FnOnce(parse::StepCursor<'c, 'a>) -> syn::Result<(R, buffer::Cursor<'c>)>,
    {
        todo!()
    }
    /// [parse::ParseBuffer::fork]
    pub fn fork(&self) -> Self {
        todo!()
    }
    /// [parse::ParseBuffer::error]
    pub fn error<T: fmt::Display>(&self, message: T) -> crate::Error {
        todo!()
    }
}
