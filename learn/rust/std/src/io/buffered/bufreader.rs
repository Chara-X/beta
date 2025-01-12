use std::{io, marker};
/// [io::BufReader]
pub struct BufReader<R: ?Sized> {
    _mk: marker::PhantomData<R>,
}
impl<R: io::Read> BufReader<R> {
    /// [io::BufReader::new]
    pub fn new(inner: R) -> BufReader<R> {
        todo!()
    }
}
