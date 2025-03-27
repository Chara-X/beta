use std::{io, path};
/// [inotify::Watches]
pub struct Watches {}
impl Watches {
    /// [inotify::Watches::add]
    pub fn add<P>(
        &mut self,
        path: P,
        mask: inotify::WatchMask,
    ) -> io::Result<inotify::WatchDescriptor>
    where
        P: AsRef<path::Path>,
    {
        todo!()
    }
    /// [inotify::Watches::remove]
    pub fn remove(&mut self, wd: inotify::WatchDescriptor) -> io::Result<()> {
        todo!()
    }
}
