use super::*;
use std::cell;
use std::hint;
use std::ops;
use std::sync;
/// [sync::Mutex]
pub struct Mutex<T: ?Sized> {
    locked: atomic::AtomicBool,
    data: cell::UnsafeCell<T>,
}
impl<T> Mutex<T> {
    /// [sync::Mutex::new]
    pub const fn new(t: T) -> Mutex<T> {
        Mutex {
            locked: atomic::AtomicBool::new(false),
            data: cell::UnsafeCell::new(t),
        }
    }
}
impl<T: ?Sized> Mutex<T> {
    /// [sync::Mutex::lock]
    pub fn lock(&self) -> sync::LockResult<MutexGuard<'_, T>> {
        while self
            .locked
            .compare_exchange(
                false,
                true,
                atomic::Ordering::Acquire,
                atomic::Ordering::Relaxed,
            )
            .is_err()
        {
            hint::spin_loop();
        }
        Ok(MutexGuard { mutex: self })
    }
}
/// [sync::MutexGuard]
pub struct MutexGuard<'a, T: ?Sized + 'a> {
    mutex: &'a Mutex<T>,
}
impl<T: ?Sized> ops::Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.data.get() }
    }
}
impl<T: ?Sized> ops::DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.data.get() }
    }
}
impl<T: ?Sized> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.locked.store(false, atomic::Ordering::Release);
    }
}
