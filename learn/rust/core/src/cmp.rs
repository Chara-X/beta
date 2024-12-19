use core::cmp;
pub trait PartialEq<Rhs: ?Sized = Self>: cmp::PartialEq<Rhs> {
    fn eq(&self, other: &Rhs) -> bool {
        cmp::PartialEq::eq(self, other)
    }
}
pub trait Ord: cmp::Ord {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        cmp::Ord::cmp(self, other)
    }
}
pub fn max<T: Ord>(v1: T, v2: T) -> T {
    cmp::max(v1, v2)
}
pub fn max_by<T, F>(v1: T, v2: T, compare: F) -> T
where
    F: Fn(&T, &T) -> cmp::Ordering,
{
    cmp::max_by(v1, v2, compare)
}
pub fn max_by_key<T, B, F>(v1: T, v2: T, f: F) -> T
where
    F: Fn(&T) -> B,
    B: Ord,
{
    cmp::max_by_key(v1, v2, f)
}
