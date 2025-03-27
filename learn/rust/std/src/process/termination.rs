use std::process;
/// [process::Termination]
pub trait Termination {
    /// [process::Termination::report]
    fn report(self) -> process::ExitCode;
}
