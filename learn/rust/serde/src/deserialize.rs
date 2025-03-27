use super::*;
/// [serde::Deserialize]
pub trait Deserialize<'de>: Sized {
    /// [serde::Deserialize]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>;
}
