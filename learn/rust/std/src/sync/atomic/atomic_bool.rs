use super::*;
/// [std::sync::atomic::AtomicBool]
pub struct AtomicBool {}
impl AtomicBool {
    /// [std::sync::atomic::AtomicBool::new]
    pub const fn new(v: bool) -> AtomicBool {
        todo!()
    }
    /// [std::sync::atomic::AtomicBool::load]
    pub fn load(&self, order: Ordering) -> bool {
        todo!()
    }
    /// [std::sync::atomic::AtomicBool::store]
    pub fn store(&self, val: bool, order: Ordering) {
        todo!()
    }
    /// [std::sync::atomic::AtomicBool::swap]
    pub fn swap(&self, val: bool, order: Ordering) -> bool {
        todo!()
    }
    /// [std::sync::atomic::AtomicBool::compare_exchange]
    pub fn compare_exchange(
        &self,
        current: bool,
        new: bool,
        success: Ordering,
        failure: Ordering,
    ) -> Result<bool, bool> {
        todo!()
    }
    /// [std::sync::atomic::AtomicBool::fetch_and]
    pub fn fetch_and(&self, val: bool, order: Ordering) -> bool {
        todo!()
    }
    /// [std::sync::atomic::AtomicBool::fetch_or]
    pub fn fetch_or(&self, val: bool, order: Ordering) -> bool {
        todo!()
    }
    /// [std::sync::atomic::AtomicBool::fetch_not]
    pub fn fetch_not(&self, order: Ordering) -> bool {
        todo!()
    }
    /// [std::sync::atomic::AtomicBool::fetch_xor]
    pub fn fetch_xor(&self, val: bool, order: Ordering) -> bool {
        todo!()
    }
}
