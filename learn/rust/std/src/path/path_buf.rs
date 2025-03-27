use super::*;
use std::ffi;
/// [std::path::PathBuf]
pub struct PathBuf {}
impl PathBuf {
    /// [std::path::PathBuf::new]
    pub fn new() -> PathBuf {
        todo!()
    }
    /// [std::path::PathBuf::push]
    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        todo!()
    }
    /// [std::path::PathBuf::pop]
    pub fn pop(&mut self) -> bool {
        todo!()
    }
    /// [std::path::PathBuf::clear]
    pub fn clear(&mut self) {
        todo!()
    }
    /// [std::path::PathBuf::set_file_name]
    pub fn set_file_name<S: AsRef<ffi::OsStr>>(&mut self, file_name: S) {
        todo!()
    }
    /// [std::path::PathBuf::set_extension]
    pub fn set_extension<S: AsRef<ffi::OsStr>>(&mut self, extension: S) -> bool {
        todo!()
    }
}
