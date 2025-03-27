use std::marker;
use std::sync::mpsc;
/// [mpsc::Receiver]
pub struct Receiver<T> {
    _data: marker::PhantomData<T>,
}
impl<T> Receiver<T> {
    /// [mpsc::Receiver::recv]
    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        todo!()
    }
    /// [mpsc::Receiver::iter]
    pub fn iter(&self) -> mpsc::Iter<'_, T> {
        todo!()
    }
}
