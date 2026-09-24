//! Expiry for a raised toast.

use std::time::{Duration, Instant};

/// The countdown behind one toast.
///
/// A toast is rebuilt every frame and holds no state of its own, so the moment it was raised lives
/// with the application. This is that moment plus the span it is given: keep one next to every
/// raised toast, feed [`Timer::remaining`] to [`crate::Toast::progress`], and drop the toast once
/// [`Timer::is_expired`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    raised: Instant,
    duration: Duration,
}

impl Timer {
    /// How long a toast stays up unless told otherwise.
    pub const DEFAULT_DURATION: Duration = Duration::from_secs(3);

    /// Starts the countdown now, over [`Timer::DEFAULT_DURATION`].
    pub fn new() -> Self {
        Self::with_duration(Self::DEFAULT_DURATION)
    }

    /// Starts the countdown now, over `duration`.
    pub fn with_duration(duration: Duration) -> Self {
        Self {
            raised: Instant::now(),
            duration,
        }
    }

    /// The fraction of the span still to run, `1.0` down to `0.0`.
    pub fn remaining(&self, now: Instant) -> f32 {
        if self.duration.is_zero() {
            return 0.0;
        }

        let spent = self.elapsed(now).as_secs_f32() / self.duration.as_secs_f32();

        (1.0 - spent).clamp(0.0, 1.0)
    }

    pub fn is_expired(&self, now: Instant) -> bool {
        self.elapsed(now) >= self.duration
    }

    pub fn duration(&self) -> Duration {
        self.duration
    }

    fn elapsed(&self, now: Instant) -> Duration {
        now.saturating_duration_since(self.raised)
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_timer_has_its_whole_span_left() {
        let timer = Timer::new();

        assert_eq!(timer.remaining(timer.raised), 1.0);
        assert!(!timer.is_expired(timer.raised));
    }

    #[test]
    fn the_span_runs_out_exactly_at_the_duration() {
        let timer = Timer::with_duration(Duration::from_secs(4));
        let end = timer.raised + timer.duration;

        assert_eq!(timer.remaining(timer.raised + Duration::from_secs(1)), 0.75);
        assert_eq!(timer.remaining(end), 0.0);
        assert!(timer.is_expired(end));
    }

    #[test]
    fn a_toast_with_no_span_is_over_before_it_starts() {
        let timer = Timer::with_duration(Duration::ZERO);

        assert_eq!(timer.remaining(timer.raised), 0.0);
        assert!(timer.is_expired(timer.raised));
    }
}
