use std::{panic, thread};
/// [panic::catch_unwind]
pub fn catch_unwind<F: FnOnce() -> R + panic::UnwindSafe, R>(f: F) -> thread::Result<R> {
    todo!()
}
