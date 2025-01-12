//! [fs]
use std::fs;
use std::io;
use std::path;
/// [fs::OpenOptions]
pub struct OpenOptions();
impl OpenOptions {
    /// [fs::OpenOptions::new]
    pub fn new() -> Self {
        todo!()
    }
    /// [fs::OpenOptions::read]
    pub fn read(&mut self, read: bool) -> &mut Self {
        todo!()
    }
    /// [fs::OpenOptions::write]
    pub fn write(&mut self, write: bool) -> &mut Self {
        todo!()
    }
    /// [fs::OpenOptions::append]
    pub fn append(&mut self, append: bool) -> &mut Self {
        todo!()
    }
    /// [fs::OpenOptions::truncate]
    pub fn truncate(&mut self, truncate: bool) -> &mut Self {
        todo!()
    }
    /// [fs::OpenOptions::create]
    pub fn create(&mut self, create: bool) -> &mut Self {
        todo!()
    }
    /// [fs::OpenOptions::open]
    pub fn open<P: AsRef<path::Path>>(&self, path: P) -> io::Result<File> {
        todo!()
    }
}
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
