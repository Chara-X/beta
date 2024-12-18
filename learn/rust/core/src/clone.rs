use core::clone;
pub trait Clone: clone::Clone {
    fn clone(&self) -> Self {
        clone::Clone::clone(&self)
    }
}
