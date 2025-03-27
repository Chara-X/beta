use super::*;
use std::io;
use std::marker;
/// [io::LineWriter]
pub struct LineWriter<W: ?Sized + Write> {
    _data: marker::PhantomData<W>,
}
impl<W: Write> LineWriter<W> {
    /// [io::LineWriter::new]
    pub fn new(inner: W) -> LineWriter<W> {
        todo!()
    }
    /// [io::LineWriter::into_inner]
    pub fn into_inner(self) -> Result<W, io::IntoInnerError<LineWriter<W>>> {
        todo!()
    }
}
impl<W: ?Sized + Write> LineWriter<W> {
    /// [io::LineWriter::get_ref]
    pub fn get_ref(&self) -> &W {
        todo!()
    }
    /// [io::LineWriter::get_mut]
    pub fn get_mut(&mut self) -> &mut W {
        todo!()
    }
}
