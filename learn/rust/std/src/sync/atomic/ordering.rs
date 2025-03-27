/// [std::sync::atomic::Ordering]
pub enum Ordering {
    /// [std::sync::atomic::Ordering::Relaxed]
    Relaxed,
    /// [std::sync::atomic::Ordering::Acquire]
    Acquire,
    /// [std::sync::atomic::Ordering::Release]
    Release,
    /// [std::sync::atomic::Ordering::AcqRel]
    SeqCst,
}
