use super::*;
use std::iter;
/// [kube::discovery::ApiGroup]
pub struct ApiGroup {}
impl ApiGroup {
    /// [kube::discovery::ApiGroup::name]
    pub fn name(&self) -> &str {
        todo!()
    }
    /// [kube::discovery::ApiGroup::versions]
    pub fn versions(&self) -> impl Iterator<Item = &str> {
        iter::empty()
    }
    /// [kube::discovery::ApiGroup::versioned_resources]
    pub fn versioned_resources(&self, ver: &str) -> Vec<(ApiResource, ApiCapabilities)> {
        todo!()
    }
}
