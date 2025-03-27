use crate::event;
use std::sync::mpsc;
/// [notify::EventHandler]
pub trait EventHandler: Send + 'static {
    /// [notify::EventHandler::handle_event]
    fn handle_event(&mut self, event: notify::Result<event::Event>);
}
impl EventHandler for mpsc::Sender<notify::Result<event::Event>> {
    fn handle_event(&mut self, event: notify::Result<event::Event>) {
        todo!()
    }
}
impl<F> EventHandler for F
where
    F: FnMut(notify::Result<event::Event>) + Send + 'static,
{
    fn handle_event(&mut self, event: notify::Result<event::Event>) {
        todo!()
    }
}
