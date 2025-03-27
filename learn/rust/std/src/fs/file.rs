use std::fs;
use std::io;
/// [fs::File]
pub struct File {}
impl File {
    /// [fs::File::metadata]
    pub fn metadata(&self) -> io::Result<fs::Metadata> {
        todo!()
    }
    /// [fs::File::set_permissions]
    pub fn set_permissions(&self, perm: fs::Permissions) -> io::Result<()> {
        todo!()
    }
    /// [fs::File::set_len]
    pub fn set_len(&self, size: u64) -> io::Result<()> {
        todo!()
    }
}
