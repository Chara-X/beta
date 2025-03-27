use super::*;
use std::ops;
/// [std::time::Instant]
pub struct Instant();
impl Instant {
    /// [std::time::Instant::now]
    pub fn now() -> Instant {
        todo!()
    }
}
impl ops::Add<Duration> for Instant {
    type Output = Instant;
    fn add(self, rhs: Duration) -> Self::Output {
        todo!()
    }
}
impl ops::Sub<Duration> for Instant {
    type Output = Instant;
    fn sub(self, rhs: Duration) -> Self::Output {
        todo!()
    }
}
impl ops::Sub for Instant {
    type Output = Duration;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
