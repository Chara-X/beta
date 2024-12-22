use core::iter;
pub fn successors<T, F>(first: Option<T>, succ: F) -> iter::Successors<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    iter::successors(first, succ)
}
