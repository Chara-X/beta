use std::marker;
/// [tokio::io::ReadBuf]
pub struct ReadBuf<'a> {
    _data: marker::PhantomData<&'a ()>,
}
