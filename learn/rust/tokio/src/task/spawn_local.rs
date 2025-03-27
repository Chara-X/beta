use super::*;
/// [tokio::task::spawn_local]
pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    todo!()
}
