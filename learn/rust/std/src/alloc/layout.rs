use std::alloc;
/// [alloc::Layout]
pub struct Layout {}
impl Layout {
    /// [alloc::Layout::new]
    pub const fn new<T>() -> Layout {
        todo!()
    }
    /// [alloc::Layout::from_size_align]
    pub const fn from_size_align(size: usize, align: usize) -> Result<Layout, alloc::LayoutError> {
        todo!()
    }
    /// [alloc::Layout::array]
    pub const fn array<T>(n: usize) -> Result<Layout, alloc::LayoutError> {
        todo!()
    }
    /// [alloc::Layout::size]
    pub const fn size(&self) -> usize {
        todo!()
    }
    /// [alloc::Layout::align]
    pub const fn align(&self) -> usize {
        todo!()
    }
    /// [alloc::Layout::pad_to_align]
    pub const fn pad_to_align(&self) -> Layout {
        todo!()
    }
    /// [alloc::Layout::align_to]
    pub const fn align_to(&self, align: usize) -> Result<Layout, alloc::LayoutError> {
        todo!()
    }
    /// [alloc::Layout::extend]
    pub const fn extend(&self, next: Layout) -> Result<(Layout, usize), alloc::LayoutError> {
        todo!()
    }
}
