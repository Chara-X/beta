use std::iter;
/// [iter::successors]
pub fn successors<T, F>(first: Option<T>, succ: F) -> iter::Successors<T, F>
where
    F: FnMut(&T) -> Option<T>,
{
    todo!()
}
