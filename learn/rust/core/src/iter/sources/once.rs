use core::iter;
pub fn once<T>(value: T) -> iter::Once<T> {
    iter::once(value)
}
