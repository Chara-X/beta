use super::*;
use std::fmt;
/// [fmt::Display]
pub trait Display {
    /// [fmt::Display::fmt]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error>;
}
