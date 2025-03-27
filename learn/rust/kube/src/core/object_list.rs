use super::*;
use kube::core;
/// [core::ObjectList]
pub struct ObjectList<T>
where
    T: Clone,
{
    /// [core::ObjectList::types]
    pub types: TypeMeta,
    /// [core::ObjectList::metadata]
    pub metadata: core::ListMeta,
    /// [core::ObjectList::items]
    pub items: Vec<T>,
}
