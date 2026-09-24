//! Toast notifications for iced.
//!
//! A toast is a level-coloured card carrying an icon, a title, an optional body, a close button and
//! an optional duration bar. [`Stack`] lays a list of them out in a window corner.
//!
//! Timing stays with the caller, who holds a [`Timer`] per raised toast: it hands
//! [`Toast::progress`] the fraction of the lifetime that is left and says when the toast is over.

pub mod stack;
pub mod timer;
pub mod toast;

pub use stack::Stack;
pub use stack::anchor::Anchor;
pub use timer::Timer;
pub use toast::Toast;
pub use toast::level::Level;
pub use toast::style::{Catalog, Style, StyleFn, default};
