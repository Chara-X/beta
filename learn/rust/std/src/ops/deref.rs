//! [std::ops]
/// [std::ops::Deref]
pub trait Deref {
    /// [std::ops::Deref::Target]
    type Target: ?Sized;
    /// [std::ops::Deref::deref]
    fn deref(&self) -> &Self::Target;
}
/// [std::ops::DerefMut]
pub trait DerefMut: Deref {
    /// [std::ops::DerefMut::deref_mut]
    fn deref_mut(&mut self) -> &mut Self::Target;
}
