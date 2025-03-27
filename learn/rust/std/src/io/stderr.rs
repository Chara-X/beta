use std::io;
/// [io::stderr]
pub fn stderr() -> Stderr {
    todo!()
}
/// [io::Stderr]
pub struct Stderr {}
impl Stderr {
    /// [io::Stderr::lock]
    pub fn lock(&self) -> io::StderrLock<'static> {
        todo!()
    }
}
