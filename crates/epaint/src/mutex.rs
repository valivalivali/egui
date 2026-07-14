//! Wrappers around `spin` locks.

// ----------------------------------------------------------------------------

/// Provides interior mutability.
///
/// It's tailored for internal use in egui should only be used for short locks (as a guideline,
/// locks should never be held longer than a single frame). In debug builds, when a lock can't
/// be acquired within 10 seconds, we assume a deadlock and will panic.
///
/// This is a thin wrapper around [`spin::Mutex`].
pub struct Mutex<T>(spin::Mutex<T>);

/// The lock you get from [`Mutex`].
pub type MutexGuard<'a, T> = spin::MutexGuard<'a, T>;

impl<T> Mutex<T> {
    #[inline(always)]
    pub fn new(val: T) -> Self {
        Self(spin::Mutex::new(val))
    }

    /// Try to acquire the lock.
    ///
    /// ## Panics
    /// Will panic in debug builds if the lock can't be acquired within 10 seconds.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.0.lock()
    }
}

// ----------------------------------------------------------------------------

/// The lock you get from [`RwLock::read`].
pub type RwLockReadGuard<'a, T> = spin::RwLockReadGuard<'a, T>;

/// The lock you get from [`RwLock::write`].
pub type RwLockWriteGuard<'a, T> = spin::RwLockWriteGuard<'a, T>;

/// Provides interior mutability.
///
/// It's tailored for internal use in egui should only be used for short locks (as a guideline,
/// locks should never be held longer than a single frame). In debug builds, when a lock can't
/// be acquired within 10 seconds, we assume a deadlock and will panic.
///
/// This is a thin wrapper around [`spin::RwLock`].
pub struct RwLock<T: ?Sized>(spin::RwLock<T>);

impl<T> RwLock<T> {
    #[inline(always)]
    pub fn new(val: T) -> Self {
        Self(spin::RwLock::new(val))
    }
}

impl<T: ?Sized> RwLock<T> {
    /// Try to acquire read-access to the lock.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.0.read()
    }

    /// Try to acquire write-access to the lock.
    #[inline(always)]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write()
    }
}

// ----------------------------------------------------------------------------

impl<T: Default> Default for Mutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Default + ?Sized> Default for RwLock<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

// ----------------------------------------------------------------------------

impl<T> Clone for Mutex<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self::new(self.lock().clone())
    }
}
