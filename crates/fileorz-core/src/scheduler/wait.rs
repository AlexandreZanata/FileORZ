//! Interruptible interval wait (cooperative stop; never kill -9).

use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

/// Sleep up to `total`, returning `false` if `stop` becomes true.
#[must_use]
pub fn wait_interruptible(stop: &AtomicBool, total: Duration, slice: Duration) -> bool {
    let deadline = Instant::now() + total;
    let slice = slice.max(Duration::from_millis(1));
    while Instant::now() < deadline {
        if stop.load(Ordering::SeqCst) {
            return false;
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        thread::sleep(remaining.min(slice));
    }
    !stop.load(Ordering::SeqCst)
}
