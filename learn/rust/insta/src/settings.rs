use insta::internals;
use std::path;
/// [insta::Settings]
pub struct Settings {}
impl Settings {
    /// [insta::Settings::new]
    pub fn new() -> Settings {
        todo!()
    }
    /// [insta::Settings::clone_current]
    pub fn clone_current() -> Settings {
        todo!()
    }
    /// [insta::Settings::set_info]
    pub fn set_info<S: serde::Serialize>(&mut self, s: &S) {
        todo!()
    }
    /// [insta::Settings::set_description]
    pub fn set_description<S: Into<String>>(&mut self, value: S) {
        todo!()
    }
    /// [insta::Settings::set_sort_maps]
    pub fn set_sort_maps(&mut self, value: bool) {
        todo!()
    }
    /// [insta::Settings::set_snapshot_suffix]
    pub fn set_snapshot_suffix<I: Into<String>>(&mut self, suffix: I) {
        todo!()
    }
    /// [insta::Settings::set_snapshot_path]
    pub fn set_snapshot_path<P: AsRef<path::Path>>(&mut self, path: P) {
        todo!()
    }
    /// [insta::Settings::add_redaction]
    pub fn add_redaction<R: Into<internals::Redaction>>(&mut self, selector: &str, replacement: R) {
        todo!()
    }
    /// [insta::Settings::add_filter]
    pub fn add_filter<S: Into<String>>(&mut self, regex: &str, replacement: S) {
        todo!()
    }
    /// [insta::Settings::bind]
    pub fn bind<F: FnOnce() -> R, R>(&self, f: F) -> R {
        todo!()
    }
}
