//! The severity of a toast, which picks its colour and its icon.

/// What a toast is telling the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Level {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

impl Level {
    /// Every level, in rising severity — handy for demos and tests.
    pub const ALL: [Self; 4] = [Self::Info, Self::Success, Self::Warning, Self::Error];
}
