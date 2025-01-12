use std::io;
use std::marker;
/// [io::BufWriter]
pub struct BufWriter<W: ?Sized + io::Write> {
    _mk: marker::PhantomData<W>,
}
impl<W: io::Write> BufWriter<W> {
    /// [io::BufWriter::new]
    pub fn new(inner: W) -> BufWriter<W> {
        todo!()
    }
}
