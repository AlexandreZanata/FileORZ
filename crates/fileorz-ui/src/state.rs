//! Main-shell run phase — Idle / Running / Error (explicit transitions).

/// Organizer lifecycle shown on the main shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RunPhase {
    /// Not organizing.
    #[default]
    Idle,
    /// Background organizer loop is active.
    Running,
    /// Last start attempt failed (folder / spawn).
    Error,
}

/// Triggers that may change [`RunPhase`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhaseEvent {
    /// Start requested; `folder_ok` mirrors validation.
    StartAttempt { folder_ok: bool },
    /// Organizer thread failed to start after validation.
    StartFailed,
    /// User stopped the organizer.
    Stopped,
    /// Clear error feedback / return to idle.
    Cleared,
}

/// Apply a phase event. Invalid combinations are no-ops that keep the phase.
#[must_use]
pub fn transition(phase: RunPhase, event: PhaseEvent) -> RunPhase {
    match (phase, event) {
        (_, PhaseEvent::StartAttempt { folder_ok: false }) => RunPhase::Error,
        (_, PhaseEvent::StartAttempt { folder_ok: true }) => RunPhase::Running,
        (_, PhaseEvent::StartFailed) => RunPhase::Error,
        (RunPhase::Running, PhaseEvent::Stopped) => RunPhase::Idle,
        (RunPhase::Error, PhaseEvent::Cleared) => RunPhase::Idle,
        (RunPhase::Idle | RunPhase::Error, PhaseEvent::Stopped) => phase,
        (RunPhase::Running | RunPhase::Idle, PhaseEvent::Cleared) => phase,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_to_running_on_valid_start() {
        assert_eq!(
            transition(RunPhase::Idle, PhaseEvent::StartAttempt { folder_ok: true }),
            RunPhase::Running
        );
    }

    #[test]
    fn idle_to_error_on_missing_folder() {
        assert_eq!(
            transition(
                RunPhase::Idle,
                PhaseEvent::StartAttempt { folder_ok: false }
            ),
            RunPhase::Error
        );
    }

    #[test]
    fn running_to_idle_on_stop() {
        assert_eq!(
            transition(RunPhase::Running, PhaseEvent::Stopped),
            RunPhase::Idle
        );
    }

    #[test]
    fn start_failed_sets_error() {
        assert_eq!(
            transition(RunPhase::Idle, PhaseEvent::StartFailed),
            RunPhase::Error
        );
        assert_eq!(
            transition(RunPhase::Running, PhaseEvent::StartFailed),
            RunPhase::Error
        );
    }

    #[test]
    fn clear_error_returns_idle() {
        assert_eq!(
            transition(RunPhase::Error, PhaseEvent::Cleared),
            RunPhase::Idle
        );
    }
}
