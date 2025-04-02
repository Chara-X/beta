use smol::prelude::*;
use std::{pin, task, time};
/// [smol::Timer]
pub struct Timer {}
impl Timer {
    /// [smol::Timer::never]
    pub fn never() -> Timer {
        todo!()
    }
    /// [smol::Timer::after]
    pub fn after(duration: time::Duration) -> Timer {
        todo!()
    }
    /// [smol::Timer::at]
    pub fn at(instant: time::Instant) -> Timer {
        todo!()
    }
    /// [smol::Timer::interval]
    pub fn interval(period: time::Duration) -> Timer {
        todo!()
    }
    /// [smol::Timer::set_after]
    pub fn set_after(&mut self, duration: time::Duration) {
        todo!()
    }
    /// [smol::Timer::set_at]
    pub fn set_at(&mut self, instant: time::Instant) {
        todo!()
    }
    /// [smol::Timer::set_interval]
    pub fn set_interval(&mut self, period: time::Duration) {
        todo!()
    }
}
impl Future for Timer {
    type Output = time::Instant;
    fn poll(self: pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        todo!()
    }
}
impl Stream for Timer {
    type Item = time::Instant;
    fn poll_next(
        self: pin::Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> task::Poll<Option<Self::Item>> {
        todo!()
    }
}
