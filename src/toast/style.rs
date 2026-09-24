//! Theme wiring for [`Toast`](crate::Toast): one [`Style`] per [`Level`].

use crate::toast::level::Level;

use iced::theme::Palette;
use iced::{Background, Border, Color, Shadow, Theme, Vector};

/// Corner rounding of a toast card, and the radius the icon badge is derived from.
const CORNER_RADIUS: f32 = 10.0;

/// How much of the card colour bleeds through the icon badge and the close button.
const OVERLAY_ALPHA: f32 = 0.22;

/// The close button under the pointer, a touch brighter than at rest.
const OVERLAY_ALPHA_HOVERED: f32 = 0.34;

/// The duration bar sits on the card as a darker band rather than a separate colour.
const DURATION_BAR_ALPHA: f32 = 0.45;

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Level) -> Style + 'a>;

/// Lets a theme describe how a toast of each [`Level`] looks.
pub trait Catalog {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>, level: Level) -> Style;
}

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, level: Level) -> Style {
        class(self, level)
    }
}

/// The appearance of a single toast card.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    pub background: Background,
    pub title_text: Color,
    pub body_text: Color,
    pub icon_background: Color,
    pub icon: Color,
    pub close_background: Color,
    pub close_background_hovered: Color,
    pub close_icon: Color,
    pub duration_bar: Color,
    pub border: Border,
    pub shadow: Shadow,
}

impl<Theme> From<Style> for StyleFn<'_, Theme> {
    fn from(style: Style) -> Self {
        Box::new(move |_theme, _level| style)
    }
}

impl Style {
    /// Builds a card from the two colours that define it: the fill and what sits on the fill.
    ///
    /// Every other colour is a translucent derivation, so a theme only has to answer the question
    /// "what colour is an error?" to get a complete card.
    #[must_use]
    pub fn from_pair(fill: Color, on_fill: Color) -> Self {
        Self {
            background: Background::Color(fill),
            title_text: on_fill,
            body_text: on_fill.scale_alpha(0.88),
            icon_background: on_fill.scale_alpha(OVERLAY_ALPHA),
            icon: on_fill,
            close_background: on_fill.scale_alpha(OVERLAY_ALPHA),
            close_background_hovered: on_fill.scale_alpha(OVERLAY_ALPHA_HOVERED),
            close_icon: on_fill,
            duration_bar: darken(fill, DURATION_BAR_ALPHA),
            border: Border::default().rounded(CORNER_RADIUS),
            shadow: Shadow {
                color: Color::BLACK.scale_alpha(0.25),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
        }
    }
}

/// Colours a toast from the theme's own semantic palette.
#[must_use]
pub fn default(theme: &Theme, level: Level) -> Style {
    let palette = theme.palette();

    Style::from_pair(fill(&palette, level), on_fill(&palette, level))
}

fn fill(palette: &Palette, level: Level) -> Color {
    match level {
        Level::Info => palette.primary,
        Level::Success => palette.success,
        Level::Warning => palette.warning,
        Level::Error => palette.danger,
    }
}

/// Picks whichever of black or white reads better on `fill`, using the same perceived-luminance
/// weights iced's own palette generation uses.
fn on_fill(palette: &Palette, level: Level) -> Color {
    const RED_WEIGHT: f32 = 0.2126;
    const GREEN_WEIGHT: f32 = 0.7152;
    const BLUE_WEIGHT: f32 = 0.0722;
    const LIGHT_THRESHOLD: f32 = 0.6;

    let fill = fill(palette, level);
    let luminance = fill.r * RED_WEIGHT + fill.g * GREEN_WEIGHT + fill.b * BLUE_WEIGHT;

    if luminance > LIGHT_THRESHOLD {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

fn darken(color: Color, amount: f32) -> Color {
    Color {
        r: color.r * (1.0 - amount),
        g: color.g * (1.0 - amount),
        b: color.b * (1.0 - amount),
        a: color.a,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_level_takes_its_own_palette_colour() {
        let theme = Theme::Light;
        let palette = theme.palette();

        assert_eq!(
            default(&theme, Level::Error).background,
            Background::Color(palette.danger)
        );
        assert_eq!(
            default(&theme, Level::Warning).background,
            Background::Color(palette.warning)
        );
        assert_eq!(
            default(&theme, Level::Success).background,
            Background::Color(palette.success)
        );
        assert_eq!(
            default(&theme, Level::Info).background,
            Background::Color(palette.primary)
        );
    }

    #[test]
    fn the_duration_bar_is_darker_than_the_card() {
        let style = Style::from_pair(Color::from_rgb(0.5, 0.5, 0.5), Color::WHITE);
        let Background::Color(card) = style.background else {
            panic!("from_pair builds a solid background");
        };

        assert!(style.duration_bar.r < card.r);
    }

    #[test]
    fn a_constant_style_converts_into_a_class() {
        let style = Style::from_pair(Color::BLACK, Color::WHITE);
        let class: StyleFn<'_, Theme> = style.into();

        assert_eq!(class(&Theme::Dark, Level::Info), style);
    }
}
