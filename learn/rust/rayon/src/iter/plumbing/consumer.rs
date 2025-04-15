/// [rayon::iter::plumbing::Consumer]
pub trait Consumer<Item>: Send + Sized {
    /// [rayon::iter::plumbing::Consumer::Folder]
    type Folder: Folder<Item, Result = Self::Result>;
    /// [rayon::iter::plumbing::Consumer::Reducer]
    type Reducer: Reducer<Self::Result>;
    /// [rayon::iter::plumbing::Consumer::Result]
    type Result: Send;
    /// [rayon::iter::plumbing::Consumer::split_at]
    fn split_at(self, index: usize) -> (Self, Self, Self::Reducer);
    /// [rayon::iter::plumbing::Consumer::into_folder]
    fn into_folder(self) -> Self::Folder;
    /// [rayon::iter::plumbing::Consumer::full]
    fn full(&self) -> bool;
}
/// [rayon::iter::plumbing::Folder]
pub trait Folder<Item>: Sized {
    /// [rayon::iter::plumbing::Folder::Result]
    type Result;
    /// [rayon::iter::plumbing::Folder::consume]
    fn consume(self, item: Item) -> Self;
    /// [rayon::iter::plumbing::Folder::complete]
    fn complete(self) -> Self::Result;
    /// [rayon::iter::plumbing::Folder::full]
    fn full(&self) -> bool;
}
/// [rayon::iter::plumbing::Reducer]
pub trait Reducer<Result> {
    /// [rayon::iter::plumbing::Reducer::reduce]
    fn reduce(self, left: Result, right: Result) -> Result;
}
