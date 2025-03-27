#![deny(missing_docs)]
#![allow(unsafe_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(private_bounds)]
#![allow(clippy::should_implement_trait)]
//! [std]
pub mod alloc;
pub mod any;
pub mod collections;
pub mod env;
pub mod fmt;
pub mod fs;
pub mod hint;
pub mod io;
pub mod mem;
pub mod net;
pub mod panic;
pub mod path;
pub mod primitive;
pub mod process;
pub mod str;
pub mod string;
pub mod sync;
pub mod thread;
pub mod time;
pub mod vec;
trait Sealed {}
