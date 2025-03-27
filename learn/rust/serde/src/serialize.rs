use super::*;
/// [serde::Serialize]
pub trait Serialize {
    /// [serde::Serializer]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}
