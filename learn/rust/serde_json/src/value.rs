/// [serde_json::Value]
pub enum Value {
    /// [serde_json::Value::Null]
    Null,
    /// [serde_json::Value::Bool]
    Bool(bool),
    /// [serde_json::Value::Number]
    Number(serde_json::Number),
    /// [serde_json::Value::String]
    String(String),
    /// [serde_json::Value::Array]
    Array(Vec<Value>),
    /// [serde_json::Value::Object]
    Object(serde_json::Map<String, Value>),
}
