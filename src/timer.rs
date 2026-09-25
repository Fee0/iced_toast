//! Expiry for a raised toast.

use std::time::{Duration, Instant};

/// The countdown behind one toast.
///
/// A toast is rebuilt every frame and holds no state of its own, so the moment it was raised lives
/// with the application. This is that moment plus the span it is given: keep one next to every
/// raised toast, feed [`Timer::remaining`] to [`crate::Toast::progress`], and drop the toast once
/// [`Timer::is_expired`].
///
/// The countdown also stands still while the mouse is over the card, so a toast being read is not
/// snatched away mid-sentence. [`crate::Toast::on_hover`] says when the mouse arrives and when it
/// leaves; hand those to [`Timer::hold`] and [`Timer::release`]. A timer told
/// [`Timer::pause_on_hover`] `false` ignores both and simply runs out.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timer {
    raised: Instant,
    duration: Duration,
    /// Time already spent under the cursor, which the countdown steps over.
    held_for: Duration,
    /// When the mouse arrived, for as long as it is still there.
    held_since: Option<Instant>,
    pause_on_hover: bool,
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
            held_for: Duration::ZERO,
            held_since: None,
            pause_on_hover: true,
        }
    }

    /// Whether the countdown stands still while the mouse is over the card. On by default.
    ///
    /// Turned off, [`Timer::hold`] and [`Timer::release`] do nothing and the span runs out on
    /// schedule, cursor or no cursor. Turning it off mid-hold starts the clock again there and
    /// then.
    #[must_use]
    pub fn pause_on_hover(mut self, pause: bool) -> Self {
        self.pause_on_hover = pause;

        if !pause {
            self.held_since = None;
        }

        self
    }

    /// Stops the countdown where it is, for the mouse arriving over the card.
    ///
    /// Holding an already held timer changes nothing, so a repeated arrival is harmless.
    pub fn hold(&mut self, now: Instant) {
        if !self.pause_on_hover || self.held_since.is_some() {
            return;
        }

        self.held_since = Some(now);
    }

    /// Lets the countdown run again, for the mouse leaving the card.
    ///
    /// The time under the cursor is taken off the span rather than added to it: the toast picks up
    /// exactly where it stopped. Releasing a timer that was never held changes nothing.
    pub fn release(&mut self, now: Instant) {
        if let Some(since) = self.held_since.take() {
            self.held_for += now.saturating_duration_since(since);
        }
    }

    /// Whether the countdown is standing still.
    pub fn is_held(&self) -> bool {
        self.held_since.is_some()
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
        let holding = self
            .held_since
            .map_or(Duration::ZERO, |since| now.saturating_duration_since(since));

        now.saturating_duration_since(self.raised)
            .saturating_sub(self.held_for)
            .saturating_sub(holding)
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
    fn a_held_toast_never_runs_out() {
        let mut timer = Timer::with_duration(Duration::from_secs(4));
        let start = timer.raised;

        timer.hold(start + Duration::from_secs(1));

        assert_eq!(timer.remaining(start + Duration::from_secs(1)), 0.75);
        assert_eq!(timer.remaining(start + Duration::from_secs(60)), 0.75);
        assert!(!timer.is_expired(start + Duration::from_secs(60)));
        assert!(timer.is_held());
    }

    #[test]
    fn the_span_picks_up_where_the_hold_left_it() {
        let mut timer = Timer::with_duration(Duration::from_secs(4));
        let start = timer.raised;

        timer.hold(start + Duration::from_secs(1));
        timer.release(start + Duration::from_secs(10));

        assert!(!timer.is_held());
        assert_eq!(timer.remaining(start + Duration::from_secs(11)), 0.5);
        assert!(timer.is_expired(start + Duration::from_secs(13)));
    }

    #[test]
    fn arriving_twice_over_and_leaving_unarrived_change_nothing() {
        let mut timer = Timer::with_duration(Duration::from_secs(4));
        let start = timer.raised;

        timer.release(start + Duration::from_secs(1));
        timer.hold(start + Duration::from_secs(1));
        timer.hold(start + Duration::from_secs(3));
        timer.release(start + Duration::from_secs(5));

        assert_eq!(timer.remaining(start + Duration::from_secs(6)), 0.5);
    }

    #[test]
    fn a_timer_that_does_not_pause_on_hover_ignores_the_mouse() {
        let mut timer = Timer::with_duration(Duration::from_secs(4)).pause_on_hover(false);
        let start = timer.raised;

        timer.hold(start + Duration::from_secs(1));

        assert!(!timer.is_held());
        assert_eq!(timer.remaining(start + Duration::from_secs(3)), 0.25);
        assert!(timer.is_expired(start + Duration::from_secs(4)));
    }

    #[test]
    fn dropping_the_option_mid_hold_starts_the_clock_again() {
        let mut timer = Timer::with_duration(Duration::from_secs(4));
        let start = timer.raised;

        timer.hold(start + Duration::from_secs(1));

        let timer = timer.pause_on_hover(false);

        assert!(!timer.is_held());
        assert_eq!(timer.remaining(start + Duration::from_secs(2)), 0.5);
    }

    #[test]
    fn a_toast_with_no_span_is_over_before_it_starts() {
        let timer = Timer::with_duration(Duration::ZERO);

        assert_eq!(timer.remaining(timer.raised), 0.0);
        assert!(timer.is_expired(timer.raised));
    }
}
