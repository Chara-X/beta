use super::*;
use std::{ffi, fs, io, path, process};
/// [process::Command]
pub struct Command {}
impl Command {
    /// [process::Command::new]
    pub fn new<S: AsRef<ffi::OsStr>>(program: S) -> Command {
        todo!()
    }
    /// [process::Command::args]
    pub fn args<I, S>(&mut self, args: I) -> &mut Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<ffi::OsStr>,
    {
        todo!()
    }
    /// [process::Command::envs]
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Command
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<ffi::OsStr>,
        V: AsRef<ffi::OsStr>,
    {
        todo!()
    }
    /// [process::Command::env_remove]
    pub fn env_remove<K: AsRef<ffi::OsStr>>(&mut self, key: K) -> &mut Command {
        todo!()
    }
    /// [process::Command::env_clear]
    pub fn env_clear(&mut self) -> &mut Command {
        todo!()
    }
    /// [process::Command::current_dir]
    pub fn current_dir<P: AsRef<path::Path>>(&mut self, dir: P) -> &mut Command {
        todo!()
    }
    /// [process::Command::stdin]
    pub fn stdin<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        todo!()
    }
    /// [process::Command::stdout]
    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        todo!()
    }
    /// [process::Command::stderr]
    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Command {
        todo!()
    }
    /// [process::Command::spawn]
    pub fn spawn(&mut self) -> io::Result<Child> {
        todo!()
    }
    /// [process::Command::get_program]
    pub fn get_program(&self) -> &ffi::OsStr {
        todo!()
    }
    /// [process::Command::get_args]
    pub fn get_args(&self) -> process::CommandArgs<'_> {
        todo!()
    }
    /// [process::Command::get_envs]
    pub fn get_envs(&self) -> process::CommandEnvs<'_> {
        todo!()
    }
    /// [process::Command::get_current_dir]
    pub fn get_current_dir(&self) -> Option<&path::Path> {
        todo!()
    }
}
/// [process::Stdio]
pub struct Stdio();
impl Stdio {
    /// [process::Stdio::piped]
    pub fn piped() -> Stdio {
        todo!()
    }
    /// [process::Stdio::inherit]
    pub fn inherit() -> Stdio {
        todo!()
    }
    /// [process::Stdio::null]
    pub fn null() -> Stdio {
        todo!()
    }
}
impl From<ChildStdin> for Stdio {
    fn from(value: ChildStdin) -> Self {
        todo!()
    }
}
impl From<ChildStdout> for Stdio {
    fn from(value: ChildStdout) -> Self {
        todo!()
    }
}
impl From<ChildStderr> for Stdio {
    fn from(value: ChildStderr) -> Self {
        todo!()
    }
}
impl From<io::Stdout> for Stdio {
    fn from(value: io::Stdout) -> Self {
        todo!()
    }
}
impl From<io::Stderr> for Stdio {
    fn from(value: io::Stderr) -> Self {
        todo!()
    }
}
impl From<fs::File> for Stdio {
    fn from(value: fs::File) -> Self {
        todo!()
    }
}
