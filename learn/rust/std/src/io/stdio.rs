use std::io;
/// [io::stdin]
pub fn stdin() -> Stdin {
    todo!()
}
/// [io::stdout]
pub fn stdout() -> Stdout {
    todo!()
}
/// [io::Stdin]
pub struct Stdin {}
impl Stdin {
    /// [io::Stdin::lock]
    pub fn lock(&self) -> io::StdinLock<'static> {
        todo!()
    }
    /// [io::Stdin::lines]
    pub fn lines(self) -> io::Lines<io::StdinLock<'static>> {
        todo!()
    }
}
/// [io::Stdout]
pub struct Stdout {}
impl Stdout {
    /// [io::Stdout::lock]
    pub fn lock(&self) -> io::StdoutLock<'static> {
        todo!()
    }
}
