#![deny(missing_docs)]
#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(private_bounds)]
#![allow(clippy::should_implement_trait)]
//! [inotify]
mod inotify;
mod watches;
pub use self::inotify::*;
pub use self::watches::*;
