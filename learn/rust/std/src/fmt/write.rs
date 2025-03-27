use super::*;
use std::fmt;
/// [fmt::write]
pub fn write(output: &mut dyn Write, args: Arguments<'_>) -> Result<(), fmt::Error> {
    todo!()
}
/// [fmt::Write]
pub trait Write {
    /// [fmt::Write::write_str]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error>;
}
