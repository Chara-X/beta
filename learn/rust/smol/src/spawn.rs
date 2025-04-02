use super::*;
/// [smol::spawn]
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
    todo!()
}
