use insta::internals;
use std::{error, path};
/// [insta::Snapshot]
pub struct Snapshot {}
impl Snapshot {
    /// [insta::Snapshot::from_file]
    pub fn from_file(p: &path::Path) -> Result<Snapshot, Box<dyn error::Error>> {
        todo!()
    }
    /// [insta::Snapshot::module_name]
    pub fn module_name(&self) -> &str {
        todo!()
    }
    /// [insta::Snapshot::snapshot_name]
    pub fn snapshot_name(&self) -> Option<&str> {
        todo!()
    }
    /// [insta::Snapshot::metadata]
    pub fn metadata(&self) -> &MetaData {
        todo!()
    }
    /// [insta::Snapshot::contents]
    pub fn contents(&self) -> &internals::SnapshotContents {
        todo!()
    }
    /// [insta::Snapshot::matches]
    pub fn matches(&self, other: &Self) -> bool {
        todo!()
    }
}
/// [insta::MetaData]
pub struct MetaData {}
impl MetaData {
    /// [insta::MetaData::source]
    pub fn source(&self) -> Option<&str> {
        todo!()
    }
    /// [insta::MetaData::assertion_line]
    pub fn assertion_line(&self) -> Option<u32> {
        todo!()
    }
    /// [insta::MetaData::expression]
    pub fn expression(&self) -> Option<&str> {
        todo!()
    }
    /// [insta::MetaData::description]
    pub fn description(&self) -> Option<&str> {
        todo!()
    }
}
