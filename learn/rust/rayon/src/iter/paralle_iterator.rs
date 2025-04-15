use super::*;
/// [rayon::iter::ParallelIterator]
pub trait ParallelIterator: Sized + Send {
    /// [rayon::iter::ParallelIterator::Item]
    type Item: Send;
    /// [rayon::iter::ParallelIterator::drive_unindexed]
    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>;
}
