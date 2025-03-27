use std::{ops, time::Duration};
use time::{error, formatting, parsing};
/// [time::Date]
pub struct Date {}
impl Date {
    /// [time::Date::MIN]
    pub const MIN: time::Date = time::Date::MIN;
    /// [time::Date::MAX]
    pub const MAX: time::Date = time::Date::MAX;
    /// [time::Date::from_calendar_date]
    pub const fn from_calendar_date(
        year: i32,
        month: time::Month,
        day: u8,
    ) -> Result<Self, error::ComponentRange> {
        todo!()
    }
    /// [time::Date::from_ordinal_date]
    pub const fn from_ordinal_date(year: i32, ordinal: u16) -> Result<Self, error::ComponentRange> {
        todo!()
    }
    /// [time::Date::from_iso_week_date]
    pub const fn from_iso_week_date(
        year: i32,
        week: u8,
        weekday: time::Weekday,
    ) -> Result<Self, error::ComponentRange> {
        todo!()
    }
    /// [time::Date::parse]
    pub fn parse(
        input: &str,
        description: &(impl parsing::Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        todo!()
    }
    /// [time::Date::prev_occurrence]
    pub const fn prev_occurrence(self, weekday: time::Weekday) -> Self {
        todo!()
    }
    /// [time::Date::next_occurrence]
    pub const fn next_occurrence(self, weekday: time::Weekday) -> Self {
        todo!()
    }
    /// [time::Date::nth_prev_occurrence]
    pub const fn nth_prev_occurrence(self, weekday: time::Weekday, n: u8) -> Self {
        todo!()
    }
    /// [time::Date::nth_next_occurrence]
    pub const fn nth_next_occurrence(self, weekday: time::Weekday, n: u8) -> Self {
        todo!()
    }
    /// [time::Date::to_calendar_date]
    pub const fn to_calendar_date(self) -> (i32, time::Month, u8) {
        todo!()
    }
    /// [time::Date::to_ordinal_date]
    pub const fn to_ordinal_date(self) -> (i32, u16) {
        todo!()
    }
    /// [time::Date::to_iso_week_date]
    pub const fn to_iso_week_date(self) -> (i32, u8, time::Weekday) {
        todo!()
    }
    /// [time::Date::format]
    pub fn format(
        self,
        format: &(impl formatting::Formattable + ?Sized),
    ) -> Result<String, error::Format> {
        todo!()
    }
}
impl ops::Add<Duration> for Date {
    type Output = Date;
    fn add(self, rhs: Duration) -> Self::Output {
        todo!()
    }
}
impl ops::AddAssign<Duration> for Date {
    fn add_assign(&mut self, rhs: Duration) {
        todo!()
    }
}
impl ops::Sub<Duration> for Date {
    type Output = Date;
    fn sub(self, rhs: Duration) -> Self::Output {
        todo!()
    }
}
impl ops::SubAssign<Duration> for Date {
    fn sub_assign(&mut self, rhs: Duration) {
        todo!()
    }
}
impl ops::Sub for Date {
    type Output = Duration;
    fn sub(self, rhs: Date) -> Self::Output {
        todo!()
    }
}
