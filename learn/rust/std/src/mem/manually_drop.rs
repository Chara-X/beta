use std::{ops, ptr};
/// [std::mem::ManuallyDrop]
pub struct ManuallyDrop<T>
where
    T: ?Sized,
{
    value: T,
}
impl<T> ManuallyDrop<T> {
    /// [std::mem::ManuallyDrop::new]
    pub const fn new(value: T) -> ManuallyDrop<T> {
        ManuallyDrop { value }
    }
    /// [std::mem::ManuallyDrop::take]
    pub unsafe fn take(slot: &mut ManuallyDrop<T>) -> T {
        unsafe { ptr::read(&slot.value) }
    }
    /// [std::mem::ManuallyDrop::into_inner]
    pub fn into_inner(slot: ManuallyDrop<T>) -> T {
        slot.value
    }
}
impl<T> ManuallyDrop<T>
where
    T: ?Sized,
{
    /// [std::mem::ManuallyDrop::drop]
    pub unsafe fn drop(slot: &mut ManuallyDrop<T>) {
        unsafe { ptr::drop_in_place(&mut slot.value) }
    }
}
impl<T> ops::Deref for ManuallyDrop<T>
where
    T: ?Sized,
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> ops::DerefMut for ManuallyDrop<T>
where
    T: ?Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
