/// [kube::core::GroupVersionKind]
pub struct GroupVersionKind {
    /// [kube::core::GroupVersionKind::group]
    pub group: String,
    /// [kube::core::GroupVersionKind::version]
    pub version: String,
    /// [kube::core::GroupVersionKind::kind]
    pub kind: String,
}
impl GroupVersionKind {
    /// [kube::core::GroupVersionKind::api_version]
    pub fn api_version(&self) -> String {
        todo!()
    }
}
