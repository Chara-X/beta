use std::io;
/// [io::stdout]
pub fn stdout() -> Stdout {
    todo!()
}
/// [io::Stdout]
pub struct Stdout {}
impl Stdout {
    /// [io::Stdout::lock]
    pub fn lock(&self) -> io::StdoutLock<'static> {
        todo!()
    }
}
