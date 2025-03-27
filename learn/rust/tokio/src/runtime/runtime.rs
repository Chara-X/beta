use crate::task;
use tokio::runtime;
/// [runtime::Runtime]
pub struct Runtime {}
impl Runtime {
    /// [runtime::Runtime::metrics]
    pub fn metrics(&self) -> runtime::RuntimeMetrics {
        todo!()
    }
    /// [runtime::Runtime::block_on]
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        todo!()
    }
    /// [runtime::Runtime::spawn]
    pub fn spawn<F>(&self, future: F) -> task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        todo!()
    }
    /// [runtime::Runtime::spawn_blocking]
    pub fn spawn_blocking<F, R>(&self, func: F) -> task::JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        todo!()
    }
}
