use core::iter;
pub trait Iterator: iter::Iterator {
    fn next(&mut self) -> Option<Self::Item> {
        iter::Iterator::next(self)
    }
}
