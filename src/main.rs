use iced_widget::space;

use crate::theme::Theme;

pub mod components;
pub mod theme;
pub mod widgets;

type Element<'a, Message> = iced::Element<'a, Message, Theme>;
type Task<Message> = iced::Task<Message>;

fn main() -> iced::Result {
    iced::application(
        || (Gallery::default(), Task::none()),
        Gallery::update,
        Gallery::view,
    )
    .theme(Gallery::theme)
    .title(Gallery::title)
    .run()
}

#[derive(Debug, Default)]
struct Gallery {
    theme: Theme,
}

#[derive(Debug, Clone)]
enum Message {}

impl Gallery {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {}
    }

    fn view(&self) -> Element<'_, Message> {
        space().into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        "Gallery".to_string()
    }
}
