//! [std::sync::mpsc]
mod channel;
mod receiver;
mod sender;
pub use self::channel::*;
pub use self::receiver::*;
pub use self::sender::*;
