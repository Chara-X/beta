#![deny(missing_docs)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
//! [proc_macro2]
mod group;
mod ident;
mod literal;
mod punct;
mod token_stream;
mod token_tree;
pub use group::*;
pub use ident::*;
pub use literal::*;
pub use punct::*;
pub use token_stream::*;
pub use token_tree::*;
