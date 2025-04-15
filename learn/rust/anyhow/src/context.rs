use super::*;
use std::{error, fmt};
/// [anyhow::Context]
pub trait Context<T, E>: Sealed {
    /// [anyhow::Context::context]
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static;
    /// [anyhow::Context::with_context]
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}
impl<T, E> Context<T, E> for Result<T, E>
where
    E: error::Error + Send + Sync + 'static,
{
    fn context<C>(self, context: C) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        todo!()
    }
    fn with_context<C, F>(self, f: F) -> Result<T, Error>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        todo!()
    }
}
impl<T, E> Sealed for Result<T, E> {}
