//! [std::alloc]
mod alloc;
mod dealloc;
mod global_alloc;
mod layout;
pub use self::alloc::*;
pub use self::dealloc::*;
pub use self::global_alloc::*;
pub use self::layout::*;
