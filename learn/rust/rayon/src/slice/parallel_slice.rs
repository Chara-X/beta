/// [rayon::slice::ParallelSlice]
pub trait ParallelSlice<T: Sync> {
    /// [rayon::slice::ParallelSlice::as_parallel_slice]
    fn as_parallel_slice(&self) -> &[T];
}
impl<T: Sync> ParallelSlice<T> for [T] {
    fn as_parallel_slice(&self) -> &[T] {
        self
    }
}
