/// [std::io::IsTerminal]
pub trait IsTerminal: crate::Sealed {
    /// [std::io::IsTerminal::is_terminal]
    fn is_terminal(&self) -> bool;
}
