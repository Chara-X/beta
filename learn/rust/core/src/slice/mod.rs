use core::slice;
pub struct Slice<T>([T]);
impl<T> Slice<T> {
    pub fn iter(&self) -> slice::Iter<'_, T> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
        self.0.iter_mut()
    }
}
