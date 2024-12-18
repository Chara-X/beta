use core::cmp;
pub trait PartialEq<Rhs: ?Sized = Self>: cmp::PartialEq<Rhs> {
    fn eq(&self, other: &Rhs) -> bool {
        cmp::PartialEq::eq(self, other)
    }
}
pub trait Eq: PartialEq<Self> {}
