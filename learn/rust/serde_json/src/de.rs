use serde::de;
/// [serde_json::from_str]
pub fn from_str<'a, T>(s: &'a str) -> serde_json::Result<T>
where
    T: de::Deserialize<'a>,
{
    todo!()
}
