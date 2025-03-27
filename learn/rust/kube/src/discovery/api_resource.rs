use crate::core;
/// [kube::discovery::ApiResource]
pub struct ApiResource {
    /// [kube::discovery::ApiResource::group]
    pub group: String,
    /// [kube::discovery::ApiResource::version]
    pub version: String,
    /// [kube::discovery::ApiResource::kind]
    pub kind: String,
}
impl ApiResource {
    /// [kube::discovery::ApiResource::from_gvk]
    pub fn from_gvk(gvk: &core::GroupVersionKind) -> ApiResource {
        todo!()
    }
    /// [kube::discovery::ApiResource::erase]
    pub fn erase<K>(dt: &<K as core::Resource>::DynamicType) -> ApiResource
    where
        K: core::Resource,
    {
        todo!()
    }
}
