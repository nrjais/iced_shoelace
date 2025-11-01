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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Overview,
    Badges,
    Buttons,
    ButtonGroups,
    Scrollables,
    Tooltips,
}

impl Default for Page {
    fn default() -> Self {
        Self::Overview
    }
}

impl Page {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Overview,
            Self::Badges,
            Self::Buttons,
            Self::ButtonGroups,
            Self::Scrollables,
            Self::Tooltips,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Overview => "Overview",
            Self::Badges => "Badges",
            Self::Buttons => "Buttons",
            Self::ButtonGroups => "Button Groups",
            Self::Scrollables => "Scrollables",
            Self::Tooltips => "Tooltips",
        }
    }
}

#[derive(Debug, Default)]
struct Gallery {
    theme: Theme,
    button_count: usize,
    current_page: Page,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(String),
    SwitchTheme(Theme),
    NavigateToPage(Page),
}

impl Gallery {
    fn update(&mut self, message: Message) -> Task<Message> {
        use std::io::Write;
        match message {
            Message::ButtonPressed(label) => {
                self.button_count += 1;
                let mut stdout = std::io::stdout();
                writeln!(
                    stdout,
                    "Button '{}' pressed! Total clicks: {}",
                    label, self.button_count
                )
                .ok();
                Task::none()
            }
            Message::SwitchTheme(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::NavigateToPage(page) => {
                self.current_page = page;
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        gallery::view(self.current_page, self.button_count)
    }

    fn theme(&self) -> Theme {
        self.theme
    }

    fn title(&self) -> String {
        format!("Gallery - {}", self.current_page.name())
    }
}
