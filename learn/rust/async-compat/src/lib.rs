#![deny(missing_docs)]
#![allow(unsafe_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(private_bounds)]
#![allow(clippy::should_implement_trait)]
//! [async_compat]
mod compat;
mod compat_ext;
pub use self::compat::*;
pub use self::compat_ext::*;
