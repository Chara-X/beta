use super::*;
/// [rayon::iter::IntoParallelIterator]
pub trait IntoParallelIterator {
    /// [rayon::iter::IntoParallelIterator::Iter]
    type Iter: ParallelIterator<Item = Self::Item>;
    /// [rayon::iter::IntoParallelIterator::Item]
    type Item: Send;
    /// [rayon::iter::IntoParallelIterator::into_par_iter]
    fn into_par_iter(self) -> Self::Iter;
}
