/// [notify::inotify::INotifyWatcher]
pub struct INotifyWatcher {}
impl crate::Watcher for INotifyWatcher {
    fn new<F: crate::EventHandler>(event_handler: F, config: crate::Config) -> notify::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
    fn kind() -> notify::WatcherKind
    where
        Self: Sized,
    {
        todo!()
    }
    fn watch(
        &mut self,
        path: &std::path::Path,
        recursive_mode: notify::RecursiveMode,
    ) -> notify::Result<()> {
        todo!()
    }
    fn unwatch(&mut self, path: &std::path::Path) -> notify::Result<()> {
        todo!()
    }
}
