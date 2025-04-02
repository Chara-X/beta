use super::*;
/// [async_compat::CompatExt]
pub trait CompatExt {
    /// [async_compat::CompatExt::compat]
    fn compat(self) -> Compat<Self>
    where
        Self: Sized;
    /// [async_compat::CompatExt::compat_ref]
    fn compat_ref(&self) -> Compat<&Self>;
    /// [async_compat::CompatExt::compat_mut]
    fn compat_mut(&mut self) -> Compat<&mut Self>;
}
impl<T> CompatExt for T {
    fn compat(self) -> Compat<Self>
    where
        Self: Sized,
    {
        Compat::new(self)
    }
    fn compat_ref(&self) -> Compat<&Self> {
        Compat::new(self)
    }
    fn compat_mut(&mut self) -> Compat<&mut Self> {
        Compat::new(self)
    }
}
