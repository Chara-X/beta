use super::*;
use notify::event;
use std::path;
/// [event::Event]
pub struct Event {
    /// [notify::Event::kind]
    pub kind: event::EventKind,
    /// [notify::Event::paths]
    pub paths: Vec<path::PathBuf>,
    /// [notify::Event::attrs]
    pub attrs: EventAttributes,
}
