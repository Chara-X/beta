use std::{ffi, path};
/// [std::path::Path]
pub struct Path {}
impl Path {
    /// [std::path::Path::new]
    pub fn new<S: AsRef<ffi::OsStr> + ?Sized>(s: &S) -> &Path {
        todo!()
    }
    /// [std::path::Path::iter]
    pub fn components(&self) -> path::Components<'_> {
        todo!()
    }
    /// [std::path::Path::ancestors]
    pub fn ancestors(&self) -> path::Ancestors<'_> {
        todo!()
    }
    /// [std::path::Path::parent]
    pub fn parent(&self) -> Option<&Path> {
        todo!()
    }
    /// [std::path::Path::file_name]
    pub fn file_name(&self) -> Option<&ffi::OsStr> {
        todo!()
    }
    /// [std::path::Path::file_stem]
    pub fn file_stem(&self) -> Option<&ffi::OsStr> {
        todo!()
    }
    /// [std::path::Path::extension]
    pub fn extension(&self) -> Option<&ffi::OsStr> {
        todo!()
    }
    /// [std::path::Path::starts_with]
    pub fn starts_with<P: AsRef<Path>>(&self, base: P) -> bool {
        todo!()
    }
    /// [std::path::Path::ends_with]
    pub fn ends_with<P: AsRef<Path>>(&self, child: P) -> bool {
        todo!()
    }
}
