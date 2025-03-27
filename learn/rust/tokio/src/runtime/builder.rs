use super::*;
use std::{io, time};
/// [tokio::runtime::Builder]
pub struct Builder {}
impl Builder {
    /// [tokio::runtime::Builder::new_current_thread]
    pub fn new_current_thread() -> Builder {
        todo!()
    }
    /// [tokio::runtime::Builder::new_multi_thread]
    pub fn new_multi_thread() -> Builder {
        todo!()
    }
    /// [tokio::runtime::Builder::enable_io]
    pub fn enable_io(&mut self) -> &mut Self {
        todo!()
    }
    /// [tokio::runtime::Builder::enable_time]
    pub fn enable_time(&mut self) -> &mut Self {
        todo!()
    }
    /// [tokio::runtime::Builder::worker_threads]
    pub fn worker_threads(&mut self, val: usize) -> &mut Self {
        todo!()
    }
    /// [tokio::runtime::Builder::max_blocking_threads]
    pub fn max_blocking_threads(&mut self, val: usize) -> &mut Self {
        todo!()
    }
    /// [tokio::runtime::Builder::thread_keep_alive]
    pub fn thread_keep_alive(&mut self, duration: time::Duration) -> &mut Self {
        todo!()
    }
    /// [tokio::runtime::Builder::global_queue_interval]
    pub fn global_queue_interval(&mut self, val: u32) -> &mut Self {
        todo!()
    }
    /// [tokio::runtime::Builder::event_interval]
    pub fn event_interval(&mut self, val: u32) -> &mut Self {
        todo!()
    }
    /// [tokio::runtime::Builder::on_thread_start]
    pub fn on_thread_start<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        todo!()
    }
    /// [tokio::runtime::Builder::on_thread_park]
    pub fn on_thread_park<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        todo!()
    }
    /// [tokio::runtime::Builder::on_thread_unpark]
    pub fn on_thread_unpark<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        todo!()
    }
    /// [tokio::runtime::Builder::on_thread_stop]
    pub fn on_thread_stop<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        todo!()
    }
    /// [tokio::runtime::Builder::build]
    pub fn build(&mut self) -> io::Result<Runtime> {
        todo!()
    }
}
