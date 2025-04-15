use super::*;
/// [rayon::iter::plumbing::UnindexedConsumer]
pub trait UnindexedConsumer<I>: Consumer<I> {
    /// [rayon::iter::plumbing::UnindexedConsumer::split_off_left]
    fn split_off_left(&self) -> Self;
    /// [rayon::iter::plumbing::UnindexedConsumer::to_reducer]
    fn to_reducer(&self) -> Self::Reducer;
}
