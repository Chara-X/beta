use super::*;
/// [notify::recommended_watcher]
pub fn recommended_watcher<F>(event_handler: F) -> notify::Result<notify::RecommendedWatcher>
where
    F: EventHandler,
{
    todo!()
}
