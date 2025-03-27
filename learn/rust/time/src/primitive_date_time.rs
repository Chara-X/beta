use super::{Date, Time};
/// [time::PrimitiveDateTime]
pub struct PrimitiveDateTime {}
impl PrimitiveDateTime {
    /// [time::PrimitiveDateTime::MIN]
    pub const MIN: time::PrimitiveDateTime = time::PrimitiveDateTime::MIN;
    /// [time::PrimitiveDateTime::MAX]
    pub const MAX: time::PrimitiveDateTime = time::PrimitiveDateTime::MAX;
    /// [time::PrimitiveDateTime::new]
    pub const fn new(date: Date, time: Time) -> Self {
        todo!()
    }
    /// [time::PrimitiveDateTime::date]
    pub const fn date(&self) -> Date {
        todo!()
    }
    /// [time::PrimitiveDateTime::time]
    pub const fn time(&self) -> Time {
        todo!()
    }
}
