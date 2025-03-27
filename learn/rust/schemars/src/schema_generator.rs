use super::*;
use schemars::generate;
use serde_json::map;
/// [schemars::SchemaGenerator]
pub struct SchemaGenerator {}
impl SchemaGenerator {
    /// [schemars::SchemaGenerator::new]
    pub fn new(settings: generate::SchemaSettings) -> SchemaGenerator {
        todo!()
    }
    /// [schemars::SchemaGenerator::definitions]
    pub fn definitions(&self) -> &map::Map<String, serde_json::Value> {
        todo!()
    }
    /// [schemars::SchemaGenerator::subschema_for]
    pub fn subschema_for<T: ?Sized + JsonSchema>(&mut self) -> serde_json::Value {
        todo!()
    }
    /// [schemars::SchemaGenerator::root_schema_for]
    pub fn root_schema_for<T: ?Sized + JsonSchema>(&mut self) -> serde_json::Value {
        todo!()
    }
}
