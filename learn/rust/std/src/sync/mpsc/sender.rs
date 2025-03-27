use std::marker;
use std::sync::mpsc;
/// [mpsc::Sender]
pub struct Sender<T> {
    __marker: marker::PhantomData<T>,
}
impl<T> Sender<T> {
    /// [mpsc::Sender::send]
    pub fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        todo!()
    }
}
