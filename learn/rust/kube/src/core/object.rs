use super::*;
use crate::discovery;
use kube::core;
use std::borrow;
/// [core::Object]
pub struct Object<P, U>
where
    P: Clone,
    U: Clone,
{
    /// [core::Object::types]
    pub types: Option<TypeMeta>,
    /// [core::Object::metadata]
    pub metadata: core::ObjectMeta,
    /// [core::Object::spec]
    pub spec: P,
    /// [core::Object::status]
    pub status: Option<U>,
}
impl<P, U> Object<P, U>
where
    P: Clone,
    U: Clone,
{
    /// [core::Object::new]
    pub fn new(name: &str, ar: &discovery::ApiResource, spec: P) -> Object<P, U> {
        todo!()
    }
}
impl<P, U> Resource for Object<P, U>
where
    P: Clone,
    U: Clone,
{
    type DynamicType = discovery::ApiResource;
    type Scope = DynamicResourceScope;
    fn group(dt: &Self::DynamicType) -> borrow::Cow<'_, str> {
        todo!()
    }
    fn version(dt: &Self::DynamicType) -> borrow::Cow<'_, str> {
        todo!()
    }
    fn kind(dt: &Self::DynamicType) -> borrow::Cow<'_, str> {
        todo!()
    }
    fn meta(&self) -> &core::ObjectMeta {
        todo!()
    }
    fn meta_mut(&mut self) -> &mut core::ObjectMeta {
        todo!()
    }
}
