use std::io;
/// [serde_json::to_writer]
pub fn to_writer<W, T>(writer: W, value: &T) -> serde_json::Result<()>
where
    W: io::Write,
    T: ?Sized + serde::Serialize,
{
    todo!()
}
