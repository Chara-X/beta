use kube::core;
use std::borrow;
/// [kube::core::Resource]
pub trait Resource {
    /// [kube::core::Resource::DynamicType]
    type DynamicType: Send + Sync + 'static;
    /// [kube::core::Resource::Scope]
    type Scope;
    /// [kube::core::Resource::group]
    fn group(dt: &Self::DynamicType) -> borrow::Cow<'_, str>;
    /// [kube::core::Resource::version]
    fn version(dt: &Self::DynamicType) -> borrow::Cow<'_, str>;
    /// [kube::core::Resource::kind]
    fn kind(dt: &Self::DynamicType) -> borrow::Cow<'_, str>;
    /// [kube::core::Resource::meta]
    fn meta(&self) -> &core::ObjectMeta;
    /// [kube::core::Resource::meta_mut]
    fn meta_mut(&mut self) -> &mut core::ObjectMeta;
}
/// [kube::core::ResourceScope]
pub trait ResourceScope {}
/// [kube::core::ClusterResourceScope]
pub struct ClusterResourceScope {}
impl ResourceScope for ClusterResourceScope {}
/// [kube::core::NamespaceResourceScope]
pub struct NamespaceResourceScope {}
impl ResourceScope for NamespaceResourceScope {}
/// [kube::core::SubResourceScope]
pub struct SubResourceScope {}
impl ResourceScope for SubResourceScope {}
/// [kube::core::DynamicResourceScope]
pub struct DynamicResourceScope {}
impl ResourceScope for DynamicResourceScope {}
