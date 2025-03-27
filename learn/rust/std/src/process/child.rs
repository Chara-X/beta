use std::{io, process};
/// [process::Child]
pub struct Child {
    /// [process::Child::stdin]
    pub stdin: Option<process::ChildStdin>,
    /// [process::Child::stdout]
    pub stdout: Option<process::ChildStdout>,
    /// [process::Child::stderr]
    pub stderr: Option<process::ChildStderr>,
}
impl Child {
    /// [process::Child::id]
    pub fn id(&self) -> u32 {
        todo!()
    }
    /// [process::Child::wait]
    pub fn wait(&mut self) -> io::Result<process::ExitStatus> {
        todo!()
    }
    /// [process::Child::kill]
    pub fn kill(&mut self) -> io::Result<()> {
        todo!()
    }
}
