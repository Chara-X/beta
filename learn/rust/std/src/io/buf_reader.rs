use std::{io, marker};
/// [io::BufReader]
pub struct BufReader<R: ?Sized> {
    _data: marker::PhantomData<R>,
}
impl<R: io::Read> BufReader<R> {
    /// [io::BufReader::new]
    pub fn new(inner: R) -> BufReader<R> {
        todo!()
    }
}
impl<R: ?Sized> BufReader<R> {
    /// [io::BufReader::buffer]
    pub fn buffer(&self) -> &[u8] {
        todo!()
    }
    /// [io::BufReader::get_ref]
    pub fn get_ref(&self) -> &R {
        todo!()
    }
    /// [io::BufReader::get_mut]
    pub fn get_mut(&mut self) -> &mut R {
        todo!()
    }
    /// [io::BufReader::into_inner]
    pub fn into_inner(self) -> R
    where
        R: Sized,
    {
        todo!()
    }
}
