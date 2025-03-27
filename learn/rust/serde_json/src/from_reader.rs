use std::io;
/// [serde_json::from_reader]
pub fn from_reader<R, T>(rdr: R) -> serde_json::Result<T>
where
    R: io::Read,
    T: for<'de> serde::Deserialize<'de>,
{
    todo!()
}
