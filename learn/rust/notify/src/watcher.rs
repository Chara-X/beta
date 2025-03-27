use super::*;
use std::path;
/// [notify::Watcher]
pub trait Watcher {
    /// [notify::Watcher::new]
    fn new<F: EventHandler>(event_handler: F, config: Config) -> notify::Result<Self>
    where
        Self: Sized;
    /// [notify::Watcher::kind]
    fn kind() -> notify::WatcherKind
    where
        Self: Sized;
    /// [notify::Watcher::watch]
    fn watch(
        &mut self,
        path: &path::Path,
        recursive_mode: notify::RecursiveMode,
    ) -> notify::Result<()>;
    /// [notify::Watcher::unwatch]
    fn unwatch(&mut self, path: &path::Path) -> notify::Result<()>;
}
