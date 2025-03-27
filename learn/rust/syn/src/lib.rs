#![deny(missing_docs)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
//! [syn]
pub mod buffer;
mod error;
pub mod meta;
pub mod parse;
pub mod spanned;
pub use self::error::*;
