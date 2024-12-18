use core::clone;
pub trait Clone: Sized
where
    Self: clone::Clone,
{
    fn clone(&self) -> Self {
        clone::Clone::clone(self)
    }
}
