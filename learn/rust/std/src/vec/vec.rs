use std::marker;
use std::ops;
use std::vec;
/// [vec::Vec]
pub struct Vec<T> {
    _data: marker::PhantomData<T>,
}
impl<T> Vec<T> {
    /// [vec::Vec::new]
    pub const fn new() -> Vec<T> {
        todo!()
    }
    /// [vec::Vec::len]
    pub fn len(&self) -> usize {
        todo!()
    }
    /// [vec::Vec::capacity]
    pub fn capacity(&self) -> usize {
        todo!()
    }
    /// [vec::Vec::resize_with]
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T,
    {
        todo!()
    }
    /// [vec::Vec::reserve]
    pub fn reserve(&mut self, additional: usize) {
        todo!()
    }
    /// [vec::Vec::reserve_exact]
    pub fn reserve_exact(&mut self, additional: usize) {
        todo!()
    }
    /// [vec::Vec::shrink_to]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        todo!()
    }
    /// [vec::Vec::shrink_to_fit]
    pub fn shrink_to_fit(&mut self) {
        todo!()
    }
    /// [vec::Vec::push]
    pub fn push(&mut self, value: T) {
        todo!()
    }
    /// [vec::Vec::pop]
    pub fn pop(&mut self) -> Option<T> {
        todo!()
    }
    /// [vec::Vec::append]
    pub fn append(&mut self, other: &mut Vec<T>) {
        todo!()
    }
    /// [vec::Vec::extend_from_within]
    pub fn extend_from_within<R>(&mut self, src: R)
    where
        R: ops::RangeBounds<usize>,
    {
        todo!()
    }
    /// [vec::Vec::insert]
    pub fn insert(&mut self, index: usize, element: T) {
        todo!()
    }
    /// [vec::Vec::remove]
    pub fn remove(&mut self, index: usize) -> T {
        todo!()
    }
    /// [vec::Vec::swap_remove]
    pub fn swap_remove(&mut self, index: usize) -> T {
        todo!()
    }
    /// [vec::Vec::clear]
    pub fn clear(&mut self) {
        todo!()
    }
    /// [vec::Vec::retain]
    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&T) -> bool,
    {
        todo!()
    }
    /// [vec::Vec::retain_mut]
    pub fn retain_mut<F>(&mut self, f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        todo!()
    }
    /// [vec::Vec::dedup_by]
    pub fn dedup_by<F>(&mut self, same_bucket: F)
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {
        todo!()
    }
    /// [vec::Vec::dedup_by_key]
    pub fn dedup_by_key<F, K>(&mut self, key: F)
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        todo!()
    }
    /// [vec::Vec::drain]
    pub fn drain<R>(&mut self, range: R) -> vec::Drain<'_, T>
    where
        R: ops::RangeBounds<usize>,
    {
        todo!()
    }
    /// [vec::Vec::splice]
    pub fn splice<R, I>(
        &mut self,
        range: R,
        replace_with: I,
    ) -> vec::Splice<'_, <I as IntoIterator>::IntoIter>
    where
        R: ops::RangeBounds<usize>,
        I: IntoIterator<Item = T>,
    {
        todo!()
    }
    /// [vec::Vec::leak]
    pub fn leak<'a>(self) -> &'a mut [T] {
        todo!()
    }
}
impl<T> Vec<T>
where
    T: Clone,
{
    /// [vec::Vec::resize]
    pub fn resize(&mut self, new_len: usize, value: T) {
        todo!()
    }
}
impl<T> Vec<T>
where
    T: PartialEq,
{
    /// [vec::Vec::dedup]
    pub fn dedup(&mut self) {
        todo!()
    }
}
impl<T> ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
impl<T> ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}
