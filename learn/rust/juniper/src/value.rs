/// [juniper::Value]
pub enum Value {
    /// [juniper::Value::Null]
    Null,
    /// [juniper::Value::Scalar]
    Scalar(juniper::DefaultScalarValue),
    /// [juniper::Value::List]
    List(Vec<Value>),
    /// [juniper::Value::Object]
    Object(Object),
}
/// [juniper::Object]
pub struct Object {}
impl Object {
    /// [juniper::Object::add_field]
    pub fn add_field<K>(&mut self, k: K, value: Value) -> Option<Value>
    where
        K: AsRef<str> + Into<String>,
    {
        todo!()
    }
    /// [juniper::Object::get_field_value]
    pub fn get_field_value<K: AsRef<str>>(&self, key: K) -> Option<&Value> {
        todo!()
    }
}
