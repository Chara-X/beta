use super::*;
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
