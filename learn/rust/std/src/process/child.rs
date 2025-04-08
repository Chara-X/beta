use std::{io, process};
/// [process::Child]
pub struct Child {
    /// [process::Child::stdin]
    pub stdin: Option<ChildStdin>,
    /// [process::Child::stdout]
    pub stdout: Option<ChildStdout>,
    /// [process::Child::stderr]
    pub stderr: Option<ChildStderr>,
}
impl Child {
    /// [process::Child::id]
    pub fn id(&self) -> u32 {
        todo!()
    }
    /// [process::Child::wait]
    pub fn wait(&mut self) -> io::Result<ExitStatus> {
        todo!()
    }
    /// [process::Child::wait_with_output]
    pub fn wait_with_output(self) -> io::Result<process::Output> {
        todo!()
    }
    /// [process::Child::kill]
    pub fn kill(&mut self) -> io::Result<()> {
        todo!()
    }
}
/// [process::ChildStdin]
pub struct ChildStdin {}
impl io::Write for &ChildStdin {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }
    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}
impl io::Write for ChildStdin {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!()
    }
    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}
/// [process::ChildStdout]
pub struct ChildStdout {}
impl io::Read for ChildStdout {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}
/// [process::ChildStderr]
pub struct ChildStderr {}
impl io::Read for ChildStderr {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!()
    }
}
/// [process::ExitStatus]
pub struct ExitStatus();
impl ExitStatus {
    /// [process::ExitStatus::success]
    pub fn success(&self) -> bool {
        todo!()
    }
    /// [process::ExitStatus::code]
    pub fn code(&self) -> Option<i32> {
        todo!()
    }
}
