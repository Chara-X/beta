use std::time;
/// [notify::Config]
pub struct Config {}
impl Config {
    /// [notify::Config::with_poll_interval]
    pub fn with_poll_interval(self, dur: time::Duration) -> Self {
        todo!()
    }
    /// [notify::Config::with_compare_contents]
    pub fn with_compare_contents(self, compare_contents: bool) -> Self {
        todo!()
    }
    /// [notify::Config::poll_interval]
    pub fn poll_interval(&self) -> Option<time::Duration> {
        todo!()
    }
    /// [notify::Config::compare_contents]
    pub fn compare_contents(&self) -> bool {
        todo!()
    }
}
