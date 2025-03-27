use super::*;
use std::io;
use std::marker;
/// [io::BufWriter]
pub struct BufWriter<W: ?Sized + Write> {
    _data: marker::PhantomData<W>,
}
impl<W: Write> BufWriter<W> {
    /// [io::BufWriter::new]
    pub fn new(inner: W) -> BufWriter<W> {
        todo!()
    }
    /// [io::BufWriter::into_inner]
    pub fn into_inner(self) -> Result<W, io::IntoInnerError<BufWriter<W>>> {
        todo!()
    }
}
impl<W: ?Sized + Write> BufWriter<W> {
    /// [io::BufWriter::buffer]
    pub fn buffer(&self) -> &[u8] {
        todo!()
    }
    /// [io::BufWriter::get_ref]
    pub fn get_ref(&self) -> &W {
        todo!()
    }
    /// [io::BufWriter::get_mut]
    pub fn get_mut(&mut self) -> &mut W {
        todo!()
    }
}
