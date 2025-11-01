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
#[derive(Default)]
pub enum Page {
    #[default]
    Overview,
    Badges,
    Breadcrumbs,
    Buttons,
    ButtonGroups,
    Cards,
    Checkboxes,
    Dialogs,
    Dividers,
    MenuItems,
    MenuLabels,
    Menus,
    Scrollables,
    Tooltips,
}


impl Page {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Overview,
            Self::Badges,
            Self::Breadcrumbs,
            Self::Buttons,
            Self::ButtonGroups,
            Self::Cards,
            Self::Checkboxes,
            Self::Dialogs,
            Self::Dividers,
            Self::MenuItems,
            Self::MenuLabels,
            Self::Menus,
            Self::Scrollables,
            Self::Tooltips,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Overview => "Overview",
            Self::Badges => "Badges",
            Self::Breadcrumbs => "Breadcrumbs",
            Self::Buttons => "Buttons",
            Self::ButtonGroups => "Button Groups",
            Self::Cards => "Cards",
            Self::Checkboxes => "Checkboxes",
            Self::Dialogs => "Dialogs",
            Self::Dividers => "Dividers",
            Self::MenuItems => "Menu Items",
            Self::MenuLabels => "Menu Labels",
            Self::Menus => "Menus",
            Self::Scrollables => "Scrollables",
            Self::Tooltips => "Tooltips",
        }
    }
}

#[derive(Debug, Default)]
struct Gallery {
    theme: Theme,
    current_page: Page,
    dialog_state: gallery::DialogState,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(String),
    CheckboxChanged(String, bool),
    MenuItemSelected,
    SwitchTheme(Theme),
    NavigateToPage(Page),
    Dialog(gallery::DialogMessage),
}

impl Gallery {
    fn update(&mut self, message: Message) -> Task<Message> {
        use std::io::Write;
        match message {
            Message::ButtonPressed(label) => {
                let mut stdout = std::io::stdout();
                writeln!(stdout, "Button '{}' pressed!", label).ok();
                Task::none()
            }
            Message::CheckboxChanged(label, checked) => {
                let mut stdout = std::io::stdout();
                writeln!(stdout, "Checkbox '{}' changed to: {}", label, checked).ok();
                Task::none()
            }
            Message::MenuItemSelected => {
                let mut stdout = std::io::stdout();
                writeln!(stdout, "Menu item selected").ok();
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
            Message::Dialog(msg) => {
                gallery::handle_dialog_message(&mut self.dialog_state, msg);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        gallery::view(self.current_page, &self.dialog_state)
    }

    fn theme(&self) -> Theme {
        self.theme
    }

    fn title(&self) -> String {
        format!("Gallery - {}", self.current_page.name())
    }
}
