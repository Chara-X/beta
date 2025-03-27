use super::*;
/// [kube::core::TypeMeta]
pub struct TypeMeta {
    /// [kube::core::TypeMeta::api_version]
    pub api_version: String,
    /// [kube::core::TypeMeta::kind]
    pub kind: String,
}
impl TypeMeta {
    /// [kube::core::TypeMeta::resource]
    pub fn resource<K>() -> TypeMeta
    where
        K: Resource<DynamicType = ()>,
    {
        todo!()
    }
    /// [kube::core::TypeMeta::list]
    pub fn list<K>() -> TypeMeta
    where
        K: Resource<DynamicType = ()>,
    {
        todo!()
    }
}
