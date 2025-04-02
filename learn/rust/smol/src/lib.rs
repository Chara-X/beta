#![deny(missing_docs)]
#![allow(unsafe_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(private_bounds)]
#![allow(clippy::should_implement_trait)]
//! [smol]
mod block_on;
mod spawn;
mod task;
mod timer;
mod unblock;
pub use block_on::*;
pub use spawn::*;
pub use task::*;
pub use timer::*;
pub use unblock::*;
