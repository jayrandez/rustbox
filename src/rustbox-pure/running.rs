use std::sync::atomic::{self, AtomicBool};

// The state of the RustBox is protected by the lock. Yay, global state!
static RUSTBOX_RUNNING: AtomicBool = atomic::ATOMIC_BOOL_INIT;

/// true iff RustBox is currently running. Beware of races here--don't rely on this for anything
/// critical unless you happen to know that RustBox cannot change state when it is called (a good
/// usecase would be checking to see if it's worth risking double printing backtraces to avoid
/// having them swallowed up by RustBox).
pub fn running() -> bool {
    RUSTBOX_RUNNING.load(atomic::Ordering::SeqCst)
}

// Internal RAII guard used to ensure we release the running lock whenever we acquire it.
#[allow(missing_copy_implementations)]
pub struct RunningGuard(());

pub fn run() -> Option<RunningGuard> {
    // Ensure that we are not already running and simultaneously set RUSTBOX_RUNNING using an
    // atomic swap. This ensures that contending threads don't trample each other.
    if RUSTBOX_RUNNING.swap(true, atomic::Ordering::SeqCst) {
        // The Rustbox was already running.
        None
    } else {
        // The RustBox was not already running, and now we have the lock.
        Some(RunningGuard(()))
    }
}

impl Drop for RunningGuard {
    fn drop(&mut self) {
        // Indicate that we're free now. We could probably get away with lower atomicity here,
        // but there's no reason to take that chance.
        RUSTBOX_RUNNING.store(false, atomic::Ordering::SeqCst);
    }
}
