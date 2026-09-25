//! Run with: cargo run --example demo

use std::time::Instant;

use iced::widget::{button, checkbox, column, row, text};
use iced::{Element, Length, Subscription, Theme, window};
use iced_toast::{Anchor, Level, Timer, Toast};

pub fn main() -> iced::Result {
    iced::application(Demo::default, Demo::update, Demo::view)
        .title("iced_toast")
        .subscription(Demo::subscription)
        .theme(|_: &Demo| Theme::Light)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Raise(Level),
    Dismiss(usize),
    Hover(usize, bool),
    ToggleDurationBar(bool),
    TogglePauseOnHover(bool),
    Tick,
}

struct Raised {
    id: usize,
    level: Level,
    /// Absent for an error, which waits for the close button.
    timer: Option<Timer>,
}

impl Raised {
    fn remaining(&self, now: Instant) -> Option<f32> {
        self.timer.map(|timer| timer.remaining(now))
    }

    fn is_expired(&self, now: Instant) -> bool {
        self.timer.is_some_and(|timer| timer.is_expired(now))
    }
}

struct Demo {
    raised: Vec<Raised>,
    next_id: usize,
    /// The line is only the reporting of the countdown, so turning it off changes nothing about
    /// when a toast goes — the timers below run either way.
    duration_bar: bool,
    /// Read by [`Timer::pause_on_hover`] as a toast is raised, so it applies to the next one up
    /// rather than to the ones already on screen.
    pause_on_hover: bool,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            raised: Vec::new(),
            next_id: 0,
            duration_bar: true,
            pause_on_hover: true,
        }
    }
}

impl Demo {
    fn update(&mut self, message: Message) {
        match message {
            Message::Raise(level) => {
                self.raised.push(Raised {
                    id: self.next_id,
                    level,
                    timer: (level != Level::Error)
                        .then(|| Timer::new().pause_on_hover(self.pause_on_hover)),
                });
                self.next_id += 1;
            }
            Message::Dismiss(id) => self.raised.retain(|raised| raised.id != id),
            Message::Hover(id, hovered) => {
                let now = Instant::now();

                if let Some(timer) = self
                    .raised
                    .iter_mut()
                    .find(|raised| raised.id == id)
                    .and_then(|raised| raised.timer.as_mut())
                {
                    if hovered {
                        timer.hold(now);
                    } else {
                        timer.release(now);
                    }
                }
            }
            Message::ToggleDurationBar(show) => self.duration_bar = show,
            Message::TogglePauseOnHover(pause) => self.pause_on_hover = pause,
            Message::Tick => {
                let now = Instant::now();
                self.raised.retain(|raised| !raised.is_expired(now));
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let now = Instant::now();

        let buttons = row(Level::ALL.map(|level| {
            button(text(label(level)))
                .on_press(Message::Raise(level))
                .into()
        }))
        .spacing(10);

        let bar_toggle = checkbox(self.duration_bar)
            .label("Duration bar")
            .on_toggle(Message::ToggleDurationBar);

        let hover_toggle = checkbox(self.pause_on_hover)
            .label("Keep open while hovered")
            .on_toggle(Message::TogglePauseOnHover);

        let toasts = iced_toast::Stack::new(self.raised.iter().map(|raised| {
            let mut toast = Toast::new(raised.level, label(raised.level))
                .body(body(raised.level))
                .duration_bar(self.duration_bar)
                .on_hover(
                    Message::Hover(raised.id, true),
                    Message::Hover(raised.id, false),
                )
                .on_close(Message::Dismiss(raised.id));

            if let Some(remaining) = raised.remaining(now) {
                toast = toast.progress(remaining);
            }

            toast
        }))
        .anchor(Anchor::BottomRight);

        iced::widget::stack![
            column![buttons, bar_toggle, hover_toggle]
                .spacing(12)
                .padding(20)
                .width(Length::Fill)
                .height(Length::Fill),
            toasts.into_element(),
        ]
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        // An error hangs about until it is closed; nothing about it moves.
        if !self.raised.iter().any(|raised| raised.timer.is_some()) {
            return Subscription::none();
        }

        window::frames().map(|_| Message::Tick)
    }
}

fn label(level: Level) -> &'static str {
    match level {
        Level::Info => "Info",
        Level::Success => "Success!",
        Level::Warning => "Warning!",
        Level::Error => "Error!",
    }
}

fn body(level: Level) -> &'static str {
    match level {
        Level::Info => "Here is some important information for you.",
        Level::Success => "Your action was completed successfully.",
        Level::Warning => "Please review your input and try again.",
        Level::Error => "Something went wrong. Please try again.",
    }
}
