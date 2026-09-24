//! Run with: cargo run --example demo

use std::time::Instant;

use iced::widget::{button, column, row, text};
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

#[derive(Default)]
struct Demo {
    raised: Vec<Raised>,
    next_id: usize,
}

impl Demo {
    fn update(&mut self, message: Message) {
        match message {
            Message::Raise(level) => {
                self.raised.push(Raised {
                    id: self.next_id,
                    level,
                    timer: (level != Level::Error).then(Timer::new),
                });
                self.next_id += 1;
            }
            Message::Dismiss(id) => self.raised.retain(|raised| raised.id != id),
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

        let toasts = iced_toast::Stack::new(self.raised.iter().map(|raised| {
            let mut toast = Toast::new(raised.level, label(raised.level))
                .body(body(raised.level))
                .on_close(Message::Dismiss(raised.id));

            if let Some(remaining) = raised.remaining(now) {
                toast = toast.progress(remaining);
            }

            toast
        }))
        .anchor(Anchor::BottomRight);

        iced::widget::stack![
            column![buttons]
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
