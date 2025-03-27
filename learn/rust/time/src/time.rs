use std::{ops, time::Duration};
use time::{error, formatting, parsing};
/// [time::Time]
pub struct Time {}
impl Time {
    /// [time::Time::MIDNIGHT]
    pub const MIDNIGHT: time::Time = time::Time::MIDNIGHT;
    /// [time::Time::MAX]
    pub const MAX: time::Time = time::Time::MAX;
    /// [time::Time::from_hms]
    pub const fn from_hms(hour: u8, minute: u8, second: u8) -> Result<Self, error::ComponentRange> {
        todo!()
    }
    /// [time::Time::parse]
    pub fn parse(
        input: &str,
        description: &(impl parsing::Parsable + ?Sized),
    ) -> Result<Self, error::Parse> {
        todo!()
    }
    /// [time::Time::as_hms]
    pub const fn as_hms(self) -> (u8, u8, u8) {
        todo!()
    }
    /// [time::Time::format]
    pub fn format(
        self,
        format: &(impl formatting::Formattable + ?Sized),
    ) -> Result<String, error::Format> {
        todo!()
    }
}
impl ops::Add<Duration> for Time {
    type Output = Time;
    fn add(self, rhs: Duration) -> Self::Output {
        todo!()
    }
}
impl ops::AddAssign<Duration> for Time {
    fn add_assign(&mut self, rhs: Duration) {
        todo!()
    }
}
impl ops::Sub<Duration> for Time {
    type Output = Time;
    fn sub(self, rhs: Duration) -> Self::Output {
        todo!()
    }
}
impl ops::SubAssign<Duration> for Time {
    fn sub_assign(&mut self, rhs: Duration) {
        todo!()
    }
}
impl ops::Sub for Time {
    type Output = Duration;
    fn sub(self, rhs: Time) -> Self::Output {
        todo!()
    }
}
