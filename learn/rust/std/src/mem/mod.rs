//! [std::mem]
mod align_of;
mod discriminant;
mod drop;
mod manually_drop;
mod replace;
mod size_of;
mod swap;
mod take;
pub use self::align_of::*;
pub use self::discriminant::*;
pub use self::drop::*;
pub use self::manually_drop::*;
pub use self::replace::*;
pub use self::size_of::*;
pub use self::swap::*;
pub use self::take::*;
