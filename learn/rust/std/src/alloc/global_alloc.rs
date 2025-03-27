use super::*;
/// [std::alloc::GlobalAlloc]
pub unsafe trait GlobalAlloc {
    /// [std::alloc::GlobalAlloc::alloc]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8;
    /// [std::alloc::GlobalAlloc::dealloc]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
}
