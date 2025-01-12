/// [serde_json::to_string]
pub fn to_string<T>(value: &T) -> serde_json::Result<String>
where
    T: ?Sized + serde::Serialize,
{
    todo!()
}
