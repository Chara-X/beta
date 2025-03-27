use super::*;
use crate::discovery;
use kube::core::{self, dynamic};
use std::borrow;
/// [kube::core::DynamicObject]
pub struct DynamicObject {
    /// [kube::core::DynamicObject::types]
    pub types: Option<TypeMeta>,
    /// [kube::core::DynamicObject::metadata]
    pub metadata: core::ObjectMeta,
    /// [kube::core::DynamicObject::data]
    pub data: serde_json::Value,
}
impl DynamicObject {
    /// [kube::core::DynamicObject::new]
    pub fn new(name: &str, resource: &discovery::ApiResource) -> DynamicObject {
        todo!()
    }
    /// [kube::core::DynamicObject::within]
    pub fn within(self, ns: &str) -> DynamicObject {
        todo!()
    }
    /// [kube::core::DynamicObject::data]
    pub fn data(self, data: serde_json::Value) -> DynamicObject {
        todo!()
    }
    /// [kube::core::DynamicObject::try_parse]
    pub fn try_parse<K>(self) -> Result<K, dynamic::ParseDynamicObjectError>
    where
        K: Resource + for<'a> serde::Deserialize<'a>,
    {
        todo!()
    }
}
impl Resource for DynamicObject {
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
