use super::*;
use std::{ffi, io, path, process};
/// [process::Command]
pub struct Command {}
impl Command {
    /// [process::Command::new]
    pub fn new<S: AsRef<ffi::OsStr>>(program: S) -> Command {
        todo!()
    }
    /// [process::Command::current_dir]
    pub fn current_dir<P: AsRef<path::Path>>(&mut self, dir: P) -> &mut Command {
        todo!()
    }
    /// [process::Command::arg]
    pub fn arg<S: AsRef<ffi::OsStr>>(&mut self, arg: S) -> &mut Command {
        todo!()
    }
    /// [process::Command::env]
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Command
    where
        K: AsRef<ffi::OsStr>,
        V: AsRef<ffi::OsStr>,
    {
        todo!()
    }
    /// [process::Command::stdin]
    pub fn stdin<T: Into<process::Stdio>>(&mut self, cfg: T) -> &mut Command {
        todo!()
    }
    /// [process::Command::stdout]
    pub fn stdout<T: Into<process::Stdio>>(&mut self, cfg: T) -> &mut Command {
        todo!()
    }
    /// [process::Command::stderr]
    pub fn stderr<T: Into<process::Stdio>>(&mut self, cfg: T) -> &mut Command {
        todo!()
    }
    /// [process::Command::spawn]
    pub fn spawn(&mut self) -> io::Result<Child> {
        todo!()
    }
}
