//! [env]
use std::{env, ffi, io, path};
/// [env::current_exe]
pub fn current_exe() -> io::Result<path::PathBuf> {
    todo!()
}
/// [env::current_dir]
pub fn current_dir() -> io::Result<path::PathBuf> {
    todo!()
}
/// [env::args]
pub fn args() -> env::Args {
    todo!()
}
/// [env::vars]
pub fn vars() -> env::Vars {
    todo!()
}
/// [env::var]
pub fn var<K: AsRef<ffi::OsStr>>(key: K) -> Result<String, env::VarError> {
    todo!()
}
