use std::{fmt, process};
/// [process::Termination]
pub trait Termination {
    /// [process::Termination::report]
    fn report(self) -> ExitCode;
}
/// [process::ExitCode]
pub struct ExitCode();
impl ExitCode {
    /// [process::ExitCode::SUCCESS]
    pub const SUCCESS: process::ExitCode = process::ExitCode::SUCCESS;
    /// [process::ExitCode::FAILURE]
    pub const FAILURE: process::ExitCode = process::ExitCode::FAILURE;
}
impl From<u8> for ExitCode {
    fn from(value: u8) -> Self {
        todo!()
    }
}
impl Termination for () {
    fn report(self) -> ExitCode {
        todo!()
    }
}
impl Termination for ExitCode {
    fn report(self) -> ExitCode {
        todo!()
    }
}
impl<T: Termination, E: fmt::Debug> Termination for Result<T, E> {
    fn report(self) -> ExitCode {
        todo!()
    }
}
