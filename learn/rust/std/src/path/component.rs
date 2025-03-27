use std::ffi;
/// [std::path::Component]
pub enum Component<'a> {
    /// [std::path::Component::RootDir]
    RootDir,
    /// [std::path::Component::ParentDir]
    ParentDir,
    /// [std::path::Component::CurDir]
    CurDir,
    /// [std::path::Component::Normal]
    Normal(&'a ffi::OsStr),
}
