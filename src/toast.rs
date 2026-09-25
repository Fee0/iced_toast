//! One notification card's content.

pub mod level;
pub mod style;

use crate::toast::level::Level;

/// A single notification: what happened, how bad it is, and how much of its life is left.
///
/// A toast carries no timer. [`Toast::progress`] is told how much of the lifetime remains, so the
/// application stays the one deciding when a toast appears and when it goes.
#[derive(Debug, Clone, PartialEq)]
pub struct Toast<Message> {
    pub(crate) level: Level,
    pub(crate) title: String,
    pub(crate) body: Option<String>,
    pub(crate) progress: Option<f32>,
    pub(crate) duration_bar: bool,
    /// The pair told when the mouse arrives over the card and when it leaves, kept together so that
    /// half of it can never be wired on its own — a card that is only ever held would never go.
    pub(crate) on_hover: Option<(Message, Message)>,
    pub(crate) on_close: Option<Message>,
}

impl<Message> Toast<Message> {
    pub fn new(level: Level, title: impl Into<String>) -> Self {
        Self {
            level,
            title: title.into(),
            body: None,
            progress: None,
            duration_bar: true,
            on_hover: None,
            on_close: None,
        }
    }

    /// The line under the title, wrapped over as many lines as it needs.
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// The fraction of the toast's lifetime still to run, drawn as a bar along the bottom edge.
    ///
    /// Left unset, the card has no bar — which is what a toast that never expires on its own wants.
    #[must_use]
    pub fn progress(mut self, remaining: f32) -> Self {
        self.progress = Some(remaining.clamp(0.0, 1.0));
        self
    }

    /// Whether the bar is drawn at all. On by default, so a toast given [`Toast::progress`] shows
    /// the line running down.
    ///
    /// Turned off, the countdown carries on untouched — it is the application's timer, and it still
    /// takes the toast away when it runs out — the card just doesn't say how much is left. That is
    /// the quiet toast: it goes on its own, without a line ticking away in the corner.
    #[must_use]
    pub fn duration_bar(mut self, show: bool) -> Self {
        self.duration_bar = show;
        self
    }

    /// Told when the mouse arrives over the card and when it leaves, in that order.
    ///
    /// The point of knowing is the countdown: hand `entered` to [`crate::Timer::hold`] and `left`
    /// to [`crate::Timer::release`], and a toast being read stays up until the mouse moves off it.
    /// A timer told [`crate::Timer::pause_on_hover`] `false` takes both and does nothing, so the
    /// application can wire this once and still let a card run out under the cursor.
    ///
    /// Left unset, the card does not watch the mouse at all. Either way it captures nothing:
    /// presses land where they would have.
    #[must_use]
    pub fn on_hover(mut self, entered: Message, left: Message) -> Self {
        self.on_hover = Some((entered, left));
        self
    }

    /// Without this the card has no close button, and only time can take it away.
    #[must_use]
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    pub fn level(&self) -> Level {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_outside_the_unit_range_is_clamped() {
        let toast = Toast::<()>::new(Level::Info, "Hi").progress(1.7);

        assert_eq!(toast.progress, Some(1.0));
    }

    #[test]
    fn a_toast_has_no_bar_until_it_is_given_progress() {
        assert_eq!(Toast::<()>::new(Level::Error, "Boom").progress, None);
    }

    #[test]
    fn the_bar_is_drawn_unless_it_is_turned_off() {
        assert!(Toast::<()>::new(Level::Info, "Hi").duration_bar);
        assert!(
            !Toast::<()>::new(Level::Info, "Hi")
                .duration_bar(false)
                .duration_bar
        );
    }

    #[test]
    fn a_toast_watches_the_mouse_only_once_it_is_asked_to() {
        assert_eq!(Toast::<()>::new(Level::Info, "Hi").on_hover, None);
        assert_eq!(
            Toast::new(Level::Info, "Hi").on_hover("in", "out").on_hover,
            Some(("in", "out"))
        );
    }

    #[test]
    fn turning_the_bar_off_leaves_the_countdown_alone() {
        let toast = Toast::<()>::new(Level::Info, "Hi")
            .progress(0.5)
            .duration_bar(false);

        assert_eq!(toast.progress, Some(0.5));
    }
}
