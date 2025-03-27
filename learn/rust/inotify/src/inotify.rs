use super::*;
use std::io;
/// [inotify::Inotify]
pub struct Inotify {}
impl Inotify {
    /// [inotify::Inotify::init]
    pub fn init() -> io::Result<Inotify> {
        todo!()
    }
    /// [inotify::Inotify::watches]
    pub fn watches(&self) -> Watches {
        todo!()
    }
    // /// [inotify::Inotify::read_events]
    // pub fn read_events<'a>(&mut self, buffer: &'a mut [u8]) -> io::Result<Events<'a>> {
    //     todo!()
    // }
}
