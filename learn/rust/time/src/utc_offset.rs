use time::{error, formatting, parsing};
/// [time::UtcOffset]
pub struct UtcOffset {}
impl UtcOffset {
    /// [time::UtcOffset::UTC]
    pub const UTC: time::UtcOffset = time::UtcOffset::UTC;
    /// [time::UtcOffset::from_hms]
    pub const fn from_hms(
        hours: i8,
        minutes: i8,
        seconds: i8,
    ) -> Result<Self, error::ComponentRange> {
        todo!()
    }
    /// [time::UtcOffset::parse]
    pub fn parse(
        input: &str,
        description: &(impl parsing::Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        todo!()
    }
    /// [time::UtcOffset::is_positive]
    pub const fn is_positive(self) -> bool {
        todo!()
    }
    /// [time::UtcOffset::is_negative]
    pub const fn is_negative(self) -> bool {
        todo!()
    }
    /// [time::UtcOffset::as_hms]
    pub const fn as_hms(self) -> (i8, i8, i8) {
        todo!()
    }
    /// [time::UtcOffset::format]
    pub fn format(
        self,
        format: &(impl formatting::Formattable + ?Sized),
    ) -> Result<String, error::Format> {
        todo!()
    }
}
