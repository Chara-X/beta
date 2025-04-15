/// [rayon::str::ParallelString]
pub trait ParallelString {
    /// [rayon::str::ParallelString::as_parallel_string]
    fn as_parallel_string(&self) -> &str;
}
impl ParallelString for str {
    fn as_parallel_string(&self) -> &str {
        self
    }
}
