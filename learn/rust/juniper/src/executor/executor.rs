use std::marker;
/// [juniper::executor::Executor]
pub struct Executor<'r, 'a, CtxT>
where
    CtxT: 'a,
{
    _data: marker::PhantomData<(&'r (), &'a (), CtxT)>,
}
