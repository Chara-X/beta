#![deny(missing_docs)]
#![deny(unsafe_code)]
#![allow(unused_variables)]
#![allow(clippy::new_without_default)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::module_inception)]
#![allow(private_bounds)]
#![allow(clippy::should_implement_trait)]
//! [notify]
mod config;
pub mod event;
mod event_handler;
pub mod inotify;
pub mod poll;
mod recommended_watcher;
mod watcher;
pub use self::config::*;
pub use self::event_handler::*;
pub use self::recommended_watcher::*;
pub use self::watcher::*;
