# iced_toast

Toast notifications for [iced](https://github.com/iced-rs/iced): a level-coloured card with an icon,
a close button and an animated duration bar, piled into a corner of the window.

Four levels — `Info`, `Success`, `Warning`, `Error` — each taking its colour from the theme.

## Usage

A toast is rebuilt every frame and keeps no state, so the countdown lives with the application: one
`Timer` per raised toast says how much of the lifetime is left and when the toast is over. It runs
for `Timer::DEFAULT_DURATION` (3 seconds) unless `Timer::with_duration` says otherwise.

```rust
use iced_toast::{Anchor, Level, Stack, Timer, Toast};

let timer = Timer::new();

let toasts = Stack::new([
    Toast::new(Level::Warning, "Warning!")
        .body("Please review your input and try again.")
        .progress(timer.remaining(now))
        .on_close(Message::Dismiss(id)),
])
.anchor(Anchor::BottomRight);

iced::widget::stack![content, toasts.into_element()]
```

## Hovering

A toast that runs out while it is being read is a toast that was never read. `Toast::on_hover` says
when the mouse arrives over a card and when it leaves; handing those to `Timer::hold` and
`Timer::release` stops the countdown for as long as the cursor is there, and picks it up exactly
where it stopped:

```rust
Toast::new(Level::Info, "Saved")
    .progress(timer.remaining(now))
    .on_hover(Message::Hover(id, true), Message::Hover(id, false))
```

```rust
Message::Hover(id, hovered) => {
    if let Some(timer) = self.timer_for(id) {
        if hovered { timer.hold(now) } else { timer.release(now) }
    }
}
```

A `Timer` pauses on hover by default. `Timer::new().pause_on_hover(false)` takes `hold` and
`release` and does nothing with them, so a card can be left to run out under the cursor without the
wiring changing.

The card only ever listens for the mouse arriving and leaving, so it captures nothing: presses land
where they would have.

`Stack` fills the space it is given and paints nothing outside the cards, so as the top layer of an
`iced::widget::stack` it lets presses through to the application beneath it.

Leaving `progress` unset gives a card with no bar — what a toast that only a close button can take
away wants.

The bar is also an option on its own: `.duration_bar(false)` keeps the countdown and drops only the
line. The timer is the application's either way, so the toast still goes when it runs out; it simply
does so without a line ticking away in the corner.

```rust
Toast::new(Level::Info, "Saved")
    .progress(timer.remaining(now))
    .duration_bar(false)
```

Colours come from the theme through the usual `Catalog` / `Style` pair. The bundled `default` reads
`primary`, `success`, `warning` and `danger` off the theme palette; a custom theme implements
`iced_toast::Catalog` and answers, per level, which colour a card is:

```rust
impl iced_toast::Catalog for MyTheme {
    type Class<'a> = iced_toast::StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme, level| iced_toast::Style::from_pair(fill(theme, level), Color::WHITE))
    }

    fn style(&self, class: &Self::Class<'_>, level: iced_toast::Level) -> iced_toast::Style {
        class(self, level)
    }
}
```

`Style::from_pair` derives the icon badge, the close button and the duration bar from those two
colours, so a theme only has to answer "what colour is an error?".

## Examples

```
cargo run --example demo
```
