use super::*;
use std::marker;
/// [std::thread::scope]
pub fn scope<'env, F, T>(f: F) -> T
where
    F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T,
{
    todo!()
}
/// [std::thread::Scope]
pub struct Scope<'scope, 'env: 'scope> {
    _data: marker::PhantomData<(&'scope (), &'env ())>,
}
impl<'scope> Scope<'scope, '_> {
    /// [std::thread::Scope::spawn]
    pub fn spawn<F, T>(&'scope self, f: F) -> ScopedJoinHandle<'scope, T>
    where
        F: FnOnce() -> T + Send + 'scope,
        T: Send + 'scope,
    {
        todo!()
    }
}
