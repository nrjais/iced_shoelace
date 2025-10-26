use crate::theme::Theme;

pub mod components;
pub mod gallery;
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
    button_count: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(String),
}

impl Gallery {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ButtonPressed(label) => {
                self.button_count += 1;
                println!("Button '{}' pressed! Total clicks: {}", label, self.button_count);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        gallery::view(self.button_count)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn title(&self) -> String {
        "Gallery".to_string()
    }
}
