use std::{backtrace, error, fmt, ops};
/// [anyhow::Error]
pub struct Error {}
impl Error {
    /// [anyhow::Error::new]
    pub fn new<E>(error: E) -> Self
    where
        E: error::Error + Send + Sync + 'static,
    {
        todo!()
    }
    /// [anyhow::Error::msg]
    pub fn msg<M>(message: M) -> Self
    where
        M: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        todo!()
    }
    /// [anyhow::Error::chain]
    pub fn chain(&self) -> anyhow::Chain<'_> {
        todo!()
    }
    /// [anyhow::Error::backtrace]
    pub fn backtrace(&self) -> &backtrace::Backtrace {
        todo!()
    }
    /// [anyhow::Error::context]
    pub fn context<C>(self, context: C) -> Self
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        todo!()
    }
}
impl<E> From<E> for Error
where
    E: error::Error + Send + Sync + 'static,
{
    fn from(value: E) -> Self {
        todo!()
    }
}
impl ops::Deref for Error {
    type Target = dyn error::Error + Send + Sync;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}
impl ops::DerefMut for Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}
