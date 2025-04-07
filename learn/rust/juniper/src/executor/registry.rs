use juniper::meta;
use std::collections;
/// [juniper::executor::Registry]
pub struct Registry<'r> {
    /// [juniper::executor::Registry::types]
    pub types: collections::HashMap<String, meta::MetaType<'r>>,
}
