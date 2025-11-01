use iced::Length;
use iced_widget::{Row, column, container, text};

use crate::components::button::Button;
use crate::components::scrollable;
use crate::theme::Theme;
use crate::theme::button::{ButtonSize, ButtonVariant};
use crate::{Element, Message, Page};

mod badges;
mod breadcrumbs;
mod button_groups;
mod buttons;
mod cards;
mod checkboxes;
mod dialogs;
mod dividers;
mod overview;
mod scrollables;
mod tooltips;

pub use dialogs::{DialogMessage, DialogState, handle_dialog_message};

pub fn view<'a>(current_page: Page, dialog_state: &'a DialogState) -> Element<'a, Message> {
    let content = Row::new()
        .push(navigation_sidebar(current_page))
        .push(page_content(current_page, dialog_state));

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn navigation_sidebar(current_page: Page) -> Element<'static, Message> {
    let title = text("Gallery").size(24).width(Length::Fill);

    let theme_dark = Button::new("Dark")
        .size(ButtonSize::Small)
        .variant(ButtonVariant::Default)
        .on_press(Message::SwitchTheme(Theme::Dark));

    let theme_light = Button::new("Light")
        .size(ButtonSize::Small)
        .variant(ButtonVariant::Default)
        .on_press(Message::SwitchTheme(Theme::Light));

    let theme_row = Row::with_children([theme_dark.into(), theme_light.into()]).spacing(5);

    let mut nav_buttons = column![title, theme_row].spacing(15).padding(10);

    // Add divider
    nav_buttons = nav_buttons.push(text("Pages").size(16));

    // Add navigation buttons for each page
    for page in Page::all() {
        let variant = if page == current_page {
            ButtonVariant::Primary
        } else {
            ButtonVariant::Default
        };

        let button = Button::new(page.name())
            .variant(variant)
            .size(ButtonSize::Medium)
            .on_press(Message::NavigateToPage(page));

        nav_buttons = nav_buttons.push(button);
    }

    container(nav_buttons)
        .width(200)
        .height(Length::Fill)
        .padding(10)
        .into()
}

fn page_content<'a>(page: Page, dialog_state: &'a DialogState) -> Element<'a, Message> {
    let content: Element<'a, Message> = match page {
        Page::Overview => overview::page(),
        Page::Badges => badges::page(),
        Page::Breadcrumbs => breadcrumbs::page(),
        Page::Buttons => buttons::page(),
        Page::ButtonGroups => button_groups::page(),
        Page::Cards => cards::page(),
        Page::Checkboxes => checkboxes::page(),
        Page::Dialogs => dialogs::view(dialog_state),
        Page::Dividers => dividers::page(),
        Page::Scrollables => scrollables::page(),
        Page::Tooltips => tooltips::page(),
    };

    scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
