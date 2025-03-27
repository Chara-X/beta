use std::{hash, marker, ops};
/// [std::collections::HashSet]
pub struct HashSet<T> {
    _data: marker::PhantomData<T>,
}
impl<T> HashSet<T>
where
    T: Eq + hash::Hash,
{
    /// [std::collections::HashSet::replace]
    pub fn replace(&mut self, value: T) -> Option<T> {
        todo!()
    }
    /// [std::collections::HashSet::is_disjoint]
    pub fn is_disjoint(&self, other: &HashSet<T>) -> bool {
        todo!()
    }
    /// [std::collections::HashSet::is_subset]
    pub fn is_subset(&self, other: &HashSet<T>) -> bool {
        todo!()
    }
    /// [std::collections::HashSet::is_superset]
    pub fn is_superset(&self, other: &HashSet<T>) -> bool {
        todo!()
    }
}
impl<T, const N: usize> From<[T; N]> for HashSet<T>
where
    T: Eq + hash::Hash,
{
    fn from(value: [T; N]) -> Self {
        todo!()
    }
}
impl<T> ops::BitAnd<&HashSet<T>> for &HashSet<T>
where
    T: Eq + hash::Hash + Clone,
{
    type Output = HashSet<T>;
    fn bitand(self, rhs: &HashSet<T>) -> Self::Output {
        todo!()
    }
}
impl<T> ops::BitOr<&HashSet<T>> for &HashSet<T>
where
    T: Eq + hash::Hash + Clone,
{
    type Output = HashSet<T>;
    fn bitor(self, rhs: &HashSet<T>) -> Self::Output {
        todo!()
    }
}
impl<T> ops::Sub<&HashSet<T>> for &HashSet<T>
where
    T: Eq + hash::Hash + Clone,
{
    type Output = HashSet<T>;
    fn sub(self, rhs: &HashSet<T>) -> Self::Output {
        todo!()
    }
}
impl<T> ops::BitXor<&HashSet<T>> for &HashSet<T>
where
    T: Eq + hash::Hash + Clone,
{
    type Output = HashSet<T>;
    fn bitxor(self, rhs: &HashSet<T>) -> Self::Output {
        todo!()
    }
}
