//! Background organizer worker handle.

use crate::advanced_pdf::KeywordGroups;
use crate::autodelete::TrashSink;
use crate::config::AppConfig;
use crate::scheduler::tick::run_tick;
use crate::scheduler::wait::wait_interruptible;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use thiserror::Error;

const WAIT_SLICE: Duration = Duration::from_millis(50);

#[derive(Debug, Error)]
pub enum StopError {
    #[error("worker did not stop within {0:?}")]
    Timeout(Duration),
    #[error("worker panicked")]
    Panic,
}

/// Inputs for [`OrganizerHandle::start`].
pub struct OrganizerOptions {
    pub root: PathBuf,
    pub config: AppConfig,
    pub keywords: KeywordGroups,
    pub interval: Duration,
    pub trash: Option<Arc<dyn TrashSink>>,
}

/// Cooperative start/stop for the organize loop (B-05 / B-10).
pub struct OrganizerHandle {
    stop: Arc<AtomicBool>,
    join: Option<JoinHandle<()>>,
}

impl OrganizerHandle {
    /// Spawn a worker thread: tick immediately, then sleep `interval` between ticks.
    #[must_use]
    pub fn start(opts: OrganizerOptions) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let stop_flag = Arc::clone(&stop);
        let join = thread::spawn(move || worker_loop(opts, stop_flag));
        Self {
            stop,
            join: Some(join),
        }
    }

    /// Request stop and join within `timeout` (no force-kill).
    pub fn stop(mut self, timeout: Duration) -> Result<(), StopError> {
        self.stop.store(true, Ordering::SeqCst);
        let Some(join) = self.join.take() else {
            return Ok(());
        };
        let (tx, rx) = std::sync::mpsc::channel();
        thread::spawn(move || {
            let _ = tx.send(join.join());
        });
        match rx.recv_timeout(timeout) {
            Ok(Ok(())) => Ok(()),
            Ok(Err(_)) => Err(StopError::Panic),
            Err(_) => Err(StopError::Timeout(timeout)),
        }
    }
}

fn worker_loop(opts: OrganizerOptions, stop: Arc<AtomicBool>) {
    loop {
        if stop.load(Ordering::SeqCst) {
            break;
        }
        let trash = opts.trash.as_deref();
        let _ = run_tick(
            &opts.root,
            &opts.config,
            &opts.keywords,
            SystemTime::now(),
            trash,
        );
        if !wait_interruptible(&stop, opts.interval, WAIT_SLICE) {
            break;
        }
    }
}
