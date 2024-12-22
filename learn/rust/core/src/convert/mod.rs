use core::convert;
pub trait From<T>: convert::From<T> {
    fn from(value: T) -> Self {
        convert::From::from(value)
    }
}
pub trait Into<T>: convert::Into<T> {
    fn into(self) -> T {
        convert::Into::into(self)
    }
}
