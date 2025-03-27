use super::{Date, Time, UtcOffset};
use time::error;
/// [time::OffsetDateTime]
pub struct OffsetDateTime {}
impl OffsetDateTime {
    /// [time::OffsetDateTime::UNIX_EPOCH]
    pub const UNIX_EPOCH: time::OffsetDateTime = time::OffsetDateTime::UNIX_EPOCH;
    /// [time::OffsetDateTime::new_in_offset]
    pub const fn new_in_offset(date: Date, time: Time, offset: UtcOffset) -> Self {
        todo!()
    }
    /// [time::OffsetDateTime::now_utc]
    pub fn now_utc() -> Self {
        todo!()
    }
    /// [time::OffsetDateTime::now_local]
    pub fn now_local() -> Result<Self, error::IndeterminateOffset> {
        todo!()
    }
    /// [time::OffsetDateTime::from_unix_timestamp]
    pub const fn from_unix_timestamp(timestamp: i64) -> Result<Self, error::ComponentRange> {
        todo!()
    }
    /// [time::OffsetDateTime::date]
    pub const fn date(self) -> Date {
        todo!()
    }
    /// [time::OffsetDateTime::time]
    pub const fn time(self) -> Time {
        todo!()
    }
    /// [time::OffsetDateTime::unix_timestamp]
    pub const fn unix_timestamp(self) -> i64 {
        todo!()
    }
}
