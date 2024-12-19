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
pub trait TryFrom<T>: convert::TryFrom<T> {
    fn try_from(value: T) -> Result<Self, Self::Error> {
        convert::TryFrom::try_from(value)
    }
}
pub trait TryInto<T>: convert::TryInto<T> {
    fn try_into(self) -> Result<T, <Self as convert::TryInto<T>>::Error> {
        convert::TryInto::try_into(self)
    }
}
