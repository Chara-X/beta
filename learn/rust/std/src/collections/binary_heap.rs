use std::{collections::binary_heap, marker};
/// [std::collections::BinaryHeap]
pub struct BinaryHeap<T> {
    _data: marker::PhantomData<T>,
}
impl<T> BinaryHeap<T>
where
    T: Ord,
{
    /// [std::collections::BinaryHeap::peek_mut]
    pub fn peek_mut(&mut self) -> Option<binary_heap::PeekMut<'_, T>> {
        todo!()
    }
    /// [std::collections::BinaryHeap::into_sorted_vec]
    pub fn into_sorted_vec(self) -> Vec<T> {
        todo!()
    }
}
impl<T> BinaryHeap<T> {
    /// [std::collections::BinaryHeap::peek]
    pub fn peek(&self) -> Option<&T> {
        todo!()
    }
}
impl<T, const N: usize> From<[T; N]> for BinaryHeap<T>
where
    T: Ord,
{
    fn from(value: [T; N]) -> Self {
        todo!()
    }
}
