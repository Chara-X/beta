#![deny(missing_docs)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
//! [quote]
mod to_tokens;
mod token_stream_ext;
pub use to_tokens::*;
pub use token_stream_ext::*;
