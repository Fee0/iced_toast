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
    pub(crate) on_close: Option<Message>,
}

impl<Message> Toast<Message> {
    pub fn new(level: Level, title: impl Into<String>) -> Self {
        Self {
            level,
            title: title.into(),
            body: None,
            progress: None,
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
}
