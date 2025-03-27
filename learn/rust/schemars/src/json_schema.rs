use super::*;
use std::borrow;
/// [schemars::JsonSchema]
pub trait JsonSchema {
    /// [schemars::JsonSchema::schema_name]
    fn schema_name() -> borrow::Cow<'static, str>;
    /// [schemars::JsonSchema::json_schema]
    fn json_schema(generator: &mut SchemaGenerator) -> serde_json::Value;
}
