use std::marker;
/// [std::io::Cursor]
pub struct Cursor<T> {
    _data: marker::PhantomData<T>,
}
impl<T> Cursor<T> {
    /// [std::io::Cursor::new]
    pub const fn new(inner: T) -> Cursor<T> {
        todo!()
    }
    /// [std::io::Cursor::position]
    pub const fn position(&self) -> u64 {
        todo!()
    }
    /// [std::io::Cursor::set_position]
    pub fn set_position(&mut self, pos: u64) {
        todo!()
    }
    /// [std::io::Cursor::get_ref]
    pub const fn get_ref(&self) -> &T {
        todo!()
    }
    /// [std::io::Cursor::get_mut]
    pub fn get_mut(&mut self) -> &mut T {
        todo!()
    }
    /// [std::io::Cursor::into_inner]
    pub fn into_inner(self) -> T {
        todo!()
    }
}
