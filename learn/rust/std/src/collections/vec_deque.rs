use std::marker;
/// [std::collections::VecDeque]
pub struct VecDeque<T> {
    _data: marker::PhantomData<T>,
}
impl<T> VecDeque<T> {
    /// [std::collections::VecDeque::front]
    pub fn front(&self) -> Option<&T> {
        todo!()
    }
    /// [std::collections::VecDeque::front_mut]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        todo!()
    }
    /// [std::collections::VecDeque::back]
    pub fn back(&self) -> Option<&T> {
        todo!()
    }
    /// [std::collections::VecDeque::back_mut]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        todo!()
    }
    /// [std::collections::VecDeque::push_front]
    pub fn push_front(&mut self, value: T) {
        todo!()
    }
    /// [std::collections::VecDeque::push_back]
    pub fn push_back(&mut self, value: T) {
        todo!()
    }
    /// [std::collections::VecDeque::pop_front]
    pub fn pop_front(&mut self) -> Option<T> {
        todo!()
    }
    /// [std::collections::VecDeque::pop_back]
    pub fn pop_back(&mut self) -> Option<T> {
        todo!()
    }
    /// [std::collections::VecDeque::swap_remove_front]
    pub fn swap_remove_front(&mut self, index: usize) -> Option<T> {
        todo!()
    }
    /// [std::collections::VecDeque::swap_remove_back]
    pub fn swap_remove_back(&mut self, index: usize) -> Option<T> {
        todo!()
    }
    /// [std::collections::VecDeque::make_contiguous]
    pub fn make_contiguous(&mut self) -> &mut [T] {
        todo!()
    }
    /// [std::collections::VecDeque::rotate_left]
    pub fn rotate_left(&mut self, n: usize) {
        todo!()
    }
    /// [std::collections::VecDeque::rotate_right]
    pub fn rotate_right(&mut self, n: usize) {
        todo!()
    }
    /// [std::collections::VecDeque::as_slices]
    pub fn as_slices(&self) -> (&[T], &[T]) {
        todo!()
    }
    /// [std::collections::VecDeque::as_mut_slices]
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        todo!()
    }
}
