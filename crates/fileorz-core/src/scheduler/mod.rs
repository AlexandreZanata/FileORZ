//! Interval organize worker (B-05 / B-10) — cooperative start/stop.

mod handle;
mod tick;
mod wait;

pub use handle::{OrganizerHandle, OrganizerOptions, StopError};
pub use tick::{run_tick, validate_root, TickError, TickReport};
pub use wait::wait_interruptible;

#[cfg(test)]
mod tests;
