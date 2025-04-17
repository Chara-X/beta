#![deny(missing_docs)]
#![allow(unsafe_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(private_bounds)]
#![allow(clippy::should_implement_trait)]
//! [insta]
mod dynamic_redaction;
mod settings;
mod snapshot;
mod sorted_redaction;
pub use self::dynamic_redaction::*;
pub use self::settings::*;
pub use self::snapshot::*;
pub use self::sorted_redaction::*;
