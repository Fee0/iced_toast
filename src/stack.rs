//! The pile of toast cards in a corner of the space the stack is given.

pub mod anchor;

use crate::stack::anchor::Anchor;
use crate::toast::Toast;
use crate::toast::level::Level;
use crate::toast::style::{Catalog, Style, StyleFn};

use std::rc::Rc;
use std::sync::LazyLock;

use iced::widget::{
    Button, Column, Container, MouseArea, ProgressBar, Row, Svg, Text, button, container,
    progress_bar, svg, text,
};
use iced::{Alignment, Border, Color, Element, Font, Length, Padding, font};

const INFO_SVG: &[u8] = include_bytes!("../assets/svg/info.svg");
const SUCCESS_SVG: &[u8] = include_bytes!("../assets/svg/success.svg");
const WARNING_SVG: &[u8] = include_bytes!("../assets/svg/warning.svg");
const ERROR_SVG: &[u8] = include_bytes!("../assets/svg/error.svg");
const CLOSE_SVG: &[u8] = include_bytes!("../assets/svg/close.svg");

static INFO_ICON: LazyLock<svg::Handle> = LazyLock::new(|| svg::Handle::from_memory(INFO_SVG));
static SUCCESS_ICON: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(SUCCESS_SVG));
static WARNING_ICON: LazyLock<svg::Handle> =
    LazyLock::new(|| svg::Handle::from_memory(WARNING_SVG));
static ERROR_ICON: LazyLock<svg::Handle> = LazyLock::new(|| svg::Handle::from_memory(ERROR_SVG));
static CLOSE_ICON: LazyLock<svg::Handle> = LazyLock::new(|| svg::Handle::from_memory(CLOSE_SVG));

const DEFAULT_WIDTH: f32 = 380.0;
const DEFAULT_SPACING: f32 = 10.0;
const DEFAULT_PADDING: f32 = 16.0;
const DEFAULT_TITLE_SIZE: f32 = 16.0;
const DEFAULT_BODY_SIZE: f32 = 13.0;

/// Breathing room between the edge of a card and its contents.
const CARD_PADDING: f32 = 14.0;

/// Gap between the icon badge, the text column and the close button.
const CARD_SPACING: f32 = 12.0;

/// Gap between the title and the body line.
const TEXT_SPACING: f32 = 3.0;

const ICON_BADGE_SIZE: f32 = 34.0;
const ICON_SIZE: f32 = 22.0;
const CLOSE_BUTTON_SIZE: f32 = 28.0;
const CLOSE_ICON_SIZE: f32 = 17.0;
const CLOSE_CORNER_RADIUS: f32 = 6.0;

/// Thickness of the duration bar along the bottom edge.
const DURATION_BAR_GIRTH: f32 = 4.0;

/// A list of [`Toast`]s piled into one corner of the space the stack is given.
///
/// The stack fills that space but paints nothing outside the cards, so it can be the top layer of
/// an [`iced::widget::stack`] without swallowing the presses meant for what is beneath it.
pub struct Stack<'a, Message, Theme = iced::Theme>
where
    Theme: Catalog,
{
    toasts: Vec<Toast<Message>>,
    width: f32,
    spacing: f32,
    padding: Padding,
    anchor: Anchor,
    font: Font,
    title_size: f32,
    body_size: f32,
    /// Shared into every child style closure, which is why it is behind an [`Rc`]: a `StyleFn` is
    /// not `Clone`.
    class: Rc<<Theme as Catalog>::Class<'a>>,
}

impl<'a, Message, Theme> Stack<'a, Message, Theme>
where
    Theme: Catalog,
{
    /// Takes the toasts oldest first; the newest ends up against the anchored corner.
    pub fn new(toasts: impl IntoIterator<Item = Toast<Message>>) -> Self {
        Self {
            toasts: toasts.into_iter().collect(),
            width: DEFAULT_WIDTH,
            spacing: DEFAULT_SPACING,
            padding: Padding::new(DEFAULT_PADDING),
            anchor: Anchor::default(),
            font: Font::DEFAULT,
            title_size: DEFAULT_TITLE_SIZE,
            body_size: DEFAULT_BODY_SIZE,
            class: Rc::new(Theme::default()),
        }
    }

    /// Width of one card. The stack itself always fills what it is given.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Inset from the edges of the space the stack fills.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    #[must_use]
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    /// The body font. The title takes the same family in bold.
    #[must_use]
    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }

    #[must_use]
    pub fn title_size(mut self, size: f32) -> Self {
        self.title_size = size;
        self
    }

    #[must_use]
    pub fn body_size(mut self, size: f32) -> Self {
        self.body_size = size;
        self
    }

    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Level) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = Rc::new((Box::new(style) as StyleFn<'a, Theme>).into());
        self
    }

    #[must_use]
    pub fn class(mut self, class: <Theme as Catalog>::Class<'a>) -> Self {
        self.class = Rc::new(class);
        self
    }
}

impl<'a, Message, Theme> Stack<'a, Message, Theme>
where
    Message: Clone + 'a,
    Theme: Catalog
        + container::Catalog
        + button::Catalog
        + text::Catalog
        + svg::Catalog
        + progress_bar::Catalog
        + 'a,
    for<'b> <Theme as container::Catalog>::Class<'b>: From<container::StyleFn<'b, Theme>>,
    for<'b> <Theme as button::Catalog>::Class<'b>: From<button::StyleFn<'b, Theme>>,
    for<'b> <Theme as text::Catalog>::Class<'b>: From<text::StyleFn<'b, Theme>>,
    for<'b> <Theme as svg::Catalog>::Class<'b>: From<svg::StyleFn<'b, Theme>>,
    for<'b> <Theme as progress_bar::Catalog>::Class<'b>: From<progress_bar::StyleFn<'b, Theme>>,
{
    pub fn into_element(self) -> Element<'a, Message, Theme> {
        let Self {
            toasts,
            width,
            spacing,
            padding,
            anchor,
            font,
            title_size,
            body_size,
            class,
        } = self;

        let mut cards: Vec<Element<'a, Message, Theme>> = toasts
            .into_iter()
            .map(|toast| card(toast, width, font, title_size, body_size, Rc::clone(&class)))
            .collect();

        if anchor.reverses() {
            cards.reverse();
        }

        Container::new(Column::with_children(cards).spacing(spacing))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(padding)
            .align_x(anchor.horizontal())
            .align_y(anchor.vertical())
            .into()
    }
}

impl<'a, Message, Theme> From<Stack<'a, Message, Theme>> for Element<'a, Message, Theme>
where
    Message: Clone + 'a,
    Theme: Catalog
        + container::Catalog
        + button::Catalog
        + text::Catalog
        + svg::Catalog
        + progress_bar::Catalog
        + 'a,
    for<'b> <Theme as container::Catalog>::Class<'b>: From<container::StyleFn<'b, Theme>>,
    for<'b> <Theme as button::Catalog>::Class<'b>: From<button::StyleFn<'b, Theme>>,
    for<'b> <Theme as text::Catalog>::Class<'b>: From<text::StyleFn<'b, Theme>>,
    for<'b> <Theme as svg::Catalog>::Class<'b>: From<svg::StyleFn<'b, Theme>>,
    for<'b> <Theme as progress_bar::Catalog>::Class<'b>: From<progress_bar::StyleFn<'b, Theme>>,
{
    fn from(stack: Stack<'a, Message, Theme>) -> Self {
        stack.into_element()
    }
}

fn card<'a, Message, Theme>(
    toast: Toast<Message>,
    width: f32,
    font: Font,
    title_size: f32,
    body_size: f32,
    class: Rc<<Theme as Catalog>::Class<'a>>,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
    Theme: Catalog
        + container::Catalog
        + button::Catalog
        + text::Catalog
        + svg::Catalog
        + progress_bar::Catalog
        + 'a,
    for<'b> <Theme as container::Catalog>::Class<'b>: From<container::StyleFn<'b, Theme>>,
    for<'b> <Theme as button::Catalog>::Class<'b>: From<button::StyleFn<'b, Theme>>,
    for<'b> <Theme as text::Catalog>::Class<'b>: From<text::StyleFn<'b, Theme>>,
    for<'b> <Theme as svg::Catalog>::Class<'b>: From<svg::StyleFn<'b, Theme>>,
    for<'b> <Theme as progress_bar::Catalog>::Class<'b>: From<progress_bar::StyleFn<'b, Theme>>,
{
    let Toast {
        level,
        title,
        body,
        progress,
        duration_bar: show_duration_bar,
        on_hover,
        on_close,
    } = toast;

    let title_font = Font {
        weight: font::Weight::Bold,
        ..font
    };

    let mut text_column = Column::new().spacing(TEXT_SPACING).push(
        Text::new(title)
            .size(title_size)
            .font(title_font)
            .style(text_styler(Rc::clone(&class), level, |style| {
                style.title_text
            })),
    );

    if let Some(body) = body {
        text_column = text_column.push(Text::new(body).size(body_size).font(font).style(
            text_styler(Rc::clone(&class), level, |style| style.body_text),
        ));
    }

    let mut row = Row::new()
        .spacing(CARD_SPACING)
        .align_y(Alignment::Start)
        .push(badge(level, Rc::clone(&class)))
        .push(text_column.width(Length::Fill));

    if let Some(on_close) = on_close {
        row = row.push(close_button(on_close, level, Rc::clone(&class)));
    }

    let mut content = Column::new().push(Container::new(row).padding(CARD_PADDING));

    // A toast whose bar is turned off still runs on the application's timer; it just doesn't draw
    // the line.
    if let Some(progress) = progress.filter(|_| show_duration_bar) {
        content = content.push(duration_bar(progress, level, Rc::clone(&class)));
    }

    let card: Element<'a, Message, Theme> = Container::new(content)
        .width(width)
        .clip(true)
        .style(move |theme: &Theme| {
            let style = Catalog::style(theme, class.as_ref(), level);

            container::Style {
                text_color: Some(style.title_text),
                background: Some(style.background),
                border: style.border,
                shadow: style.shadow,
                snap: true,
            }
        })
        .into();

    let Some((entered, left)) = on_hover else {
        return card;
    };

    // Only the arriving and the leaving are wired, and a mouse area captures nothing it was not
    // given a use for: presses still reach the close button and the application under the stack.
    MouseArea::new(card).on_enter(entered).on_exit(left).into()
}

fn badge<'a, Message, Theme>(
    level: Level,
    class: Rc<<Theme as Catalog>::Class<'a>>,
) -> Container<'a, Message, Theme>
where
    Theme: Catalog + container::Catalog + svg::Catalog + 'a,
    for<'b> <Theme as container::Catalog>::Class<'b>: From<container::StyleFn<'b, Theme>>,
    for<'b> <Theme as svg::Catalog>::Class<'b>: From<svg::StyleFn<'b, Theme>>,
{
    let icon = Svg::new(level_icon(level))
        .width(ICON_SIZE)
        .height(ICON_SIZE)
        .style(svg_styler(Rc::clone(&class), level, |style| style.icon));

    Container::new(icon)
        .center(ICON_BADGE_SIZE)
        .style(move |theme: &Theme| {
            let style = Catalog::style(theme, class.as_ref(), level);

            container::Style {
                background: Some(style.icon_background.into()),
                border: Border::default().rounded(ICON_BADGE_SIZE / 2.0),
                ..container::Style::default()
            }
        })
}

fn close_button<'a, Message, Theme>(
    on_close: Message,
    level: Level,
    class: Rc<<Theme as Catalog>::Class<'a>>,
) -> Button<'a, Message, Theme>
where
    Message: Clone + 'a,
    Theme: Catalog + container::Catalog + button::Catalog + svg::Catalog + 'a,
    for<'b> <Theme as button::Catalog>::Class<'b>: From<button::StyleFn<'b, Theme>>,
    for<'b> <Theme as svg::Catalog>::Class<'b>: From<svg::StyleFn<'b, Theme>>,
{
    let icon = Svg::new(CLOSE_ICON.clone())
        .width(CLOSE_ICON_SIZE)
        .height(CLOSE_ICON_SIZE)
        .style(svg_styler(Rc::clone(&class), level, |style| {
            style.close_icon
        }));

    Button::new(Container::new(icon).center(Length::Fill))
        .width(CLOSE_BUTTON_SIZE)
        .height(CLOSE_BUTTON_SIZE)
        .padding(0)
        .on_press(on_close)
        .style(move |theme: &Theme, status| {
            let style = Catalog::style(theme, class.as_ref(), level);
            let background = match status {
                button::Status::Hovered | button::Status::Pressed => style.close_background_hovered,
                button::Status::Active | button::Status::Disabled => style.close_background,
            };

            button::Style {
                background: Some(background.into()),
                text_color: style.close_icon,
                border: Border::default().rounded(CLOSE_CORNER_RADIUS),
                ..button::Style::default()
            }
        })
}

fn duration_bar<'a, Theme>(
    remaining: f32,
    level: Level,
    class: Rc<<Theme as Catalog>::Class<'a>>,
) -> ProgressBar<'a, Theme>
where
    Theme: Catalog + progress_bar::Catalog + 'a,
    for<'b> <Theme as progress_bar::Catalog>::Class<'b>: From<progress_bar::StyleFn<'b, Theme>>,
{
    const FULL: f32 = 1.0;

    ProgressBar::new(0.0..=FULL, remaining)
        .length(Length::Fill)
        .girth(DURATION_BAR_GIRTH)
        .style(move |theme: &Theme| {
            let style = Catalog::style(theme, class.as_ref(), level);

            progress_bar::Style {
                background: Color::TRANSPARENT.into(),
                bar: style.duration_bar.into(),
                border: Border::default(),
            }
        })
}

/// Pulls one colour out of the resolved [`Style`] for a text child, which can only be done once the
/// theme is in hand at draw time.
fn text_styler<'a, Theme>(
    class: Rc<<Theme as Catalog>::Class<'a>>,
    level: Level,
    pick: fn(Style) -> Color,
) -> impl Fn(&Theme) -> text::Style + 'a
where
    Theme: Catalog + 'a,
{
    move |theme: &Theme| text::Style {
        color: Some(pick(Catalog::style(theme, class.as_ref(), level))),
    }
}

/// The same, for an svg child, which is handed a status it has no use for.
fn svg_styler<'a, Theme>(
    class: Rc<<Theme as Catalog>::Class<'a>>,
    level: Level,
    pick: fn(Style) -> Color,
) -> impl Fn(&Theme, svg::Status) -> svg::Style + 'a
where
    Theme: Catalog + 'a,
{
    move |theme: &Theme, _status| svg::Style {
        color: Some(pick(Catalog::style(theme, class.as_ref(), level))),
    }
}

fn level_icon(level: Level) -> svg::Handle {
    match level {
        Level::Info => INFO_ICON.clone(),
        Level::Success => SUCCESS_ICON.clone(),
        Level::Warning => WARNING_ICON.clone(),
        Level::Error => ERROR_ICON.clone(),
    }
}
