use super::*;
use crate::runtime;
/// [tokio::task::LocalSet]
pub struct LocalSet {}
impl LocalSet {
    /// [tokio::task::LocalSet::new]
    pub fn new() -> LocalSet {
        todo!()
    }
    /// [tokio::task::LocalSet::block_on]
    pub fn block_on<F>(&self, rt: &runtime::Runtime, future: F) -> F::Output
    where
        F: Future,
    {
        todo!()
    }
    /// [tokio::task::LocalSet::spawn_local]
    pub fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        todo!()
    }
}
