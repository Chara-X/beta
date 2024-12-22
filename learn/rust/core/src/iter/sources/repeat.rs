use core::iter;
pub fn repeat<T: Clone>(elt: T) -> iter::Repeat<T> {
    iter::repeat(elt)
}
