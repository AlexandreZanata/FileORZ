//! Subtle non-blocking UI motion (≤3 cues).

/// Animation clocks for feedback fade + settings enter fade.
#[derive(Debug, Clone, Copy)]
pub struct Motion {
    /// 0..1 opacity for main feedback line.
    pub feedback_t: f32,
    /// 0..1 opacity for settings content enter.
    pub screen_t: f32,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            feedback_t: 1.0,
            screen_t: 1.0,
        }
    }
}

impl Motion {
    /// Start feedback fade-in.
    pub fn kick_feedback(&mut self) {
        self.feedback_t = 0.0;
    }

    /// Start settings screen fade-in.
    pub fn kick_screen(&mut self) {
        self.screen_t = 0.0;
    }

    /// Advance clocks; returns true while still animating.
    pub fn tick(&mut self) -> bool {
        let mut busy = false;
        if self.feedback_t < 1.0 {
            self.feedback_t = (self.feedback_t + 0.18).min(1.0);
            busy = self.feedback_t < 1.0;
        }
        if self.screen_t < 1.0 {
            self.screen_t = (self.screen_t + 0.22).min(1.0);
            busy |= self.screen_t < 1.0;
        }
        busy
    }

    /// True when a subscription tick is still useful.
    #[must_use]
    pub fn needs_tick(self) -> bool {
        self.feedback_t < 1.0 || self.screen_t < 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_reaches_one() {
        let mut m = Motion::default();
        m.kick_feedback();
        m.kick_screen();
        assert!(m.needs_tick());
        for _ in 0..20 {
            let _ = m.tick();
        }
        assert!(!m.needs_tick());
        assert!((m.feedback_t - 1.0).abs() < f32::EPSILON);
        assert!((m.screen_t - 1.0).abs() < f32::EPSILON);
    }
}
