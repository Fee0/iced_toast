//! Which corner a [`Stack`](crate::Stack) sits in.

use iced::alignment::{Horizontal, Vertical};

/// The corner the toasts pile up in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Anchor {
    TopLeft,
    TopRight,
    BottomLeft,
    #[default]
    BottomRight,
}

impl Anchor {
    pub fn horizontal(self) -> Horizontal {
        match self {
            Self::TopLeft | Self::BottomLeft => Horizontal::Left,
            Self::TopRight | Self::BottomRight => Horizontal::Right,
        }
    }

    pub fn vertical(self) -> Vertical {
        match self {
            Self::TopLeft | Self::TopRight => Vertical::Top,
            Self::BottomLeft | Self::BottomRight => Vertical::Bottom,
        }
    }

    /// Whether the oldest-first list has to be flipped to keep the newest toast against the edge.
    ///
    /// Piling downwards from the top puts the last of the list furthest from the corner, so the
    /// top anchors read the list backwards.
    pub fn reverses(self) -> bool {
        matches!(self, Self::TopLeft | Self::TopRight)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_top_anchors_flip_the_list() {
        assert!(Anchor::TopRight.reverses());
        assert!(!Anchor::BottomRight.reverses());
    }

    #[test]
    fn the_left_anchors_align_left() {
        assert_eq!(Anchor::BottomLeft.horizontal(), Horizontal::Left);
        assert_eq!(Anchor::BottomLeft.vertical(), Vertical::Bottom);
    }
}
