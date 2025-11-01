use crate::components::button::Button;
use crate::theme::Theme;
use crate::theme::button::ButtonVariant;
use crate::theme::container::ContainerStyleClass;
use crate::widgets::overlay::Overlay;
use crate::{Element, Message};

use iced::widget::{Column, Row, column, container, text};
use iced::{Alignment, Color, Fill};
use iced_core::Length;

#[derive(Debug, Clone, Default)]
pub struct DialogState {
    pub show_basic: bool,
    pub show_custom_width: bool,
    pub show_no_header: bool,
    pub show_scroll: bool,
}

#[derive(Debug, Clone)]
pub enum DialogMessage {
    OpenBasic,
    CloseBasic,
    OpenCustomWidth,
    CloseCustomWidth,
    OpenNoHeader,
    CloseNoHeader,
    OpenScroll,
    CloseScroll,
}

pub fn handle_dialog_message(state: &mut DialogState, message: DialogMessage) {
    match message {
        DialogMessage::OpenBasic => state.show_basic = true,
        DialogMessage::CloseBasic => state.show_basic = false,
        DialogMessage::OpenCustomWidth => state.show_custom_width = true,
        DialogMessage::CloseCustomWidth => state.show_custom_width = false,
        DialogMessage::OpenNoHeader => state.show_no_header = true,
        DialogMessage::CloseNoHeader => state.show_no_header = false,
        DialogMessage::OpenScroll => state.show_scroll = true,
        DialogMessage::CloseScroll => state.show_scroll = false,
    }
}

pub fn view(state: &DialogState) -> Element<'static, Message> {
    container(
        column![
            text("Dialogs").size(32),
            text("Dialogs display important information that requires user attention.").size(14),
            basic_dialog(state.show_basic),
            custom_width_dialog(state.show_custom_width),
            no_header_dialog(state.show_no_header),
            scroll_dialog(state.show_scroll),
        ]
        .spacing(20),
    )
    .padding(20)
    .into()
}

fn basic_dialog(show: bool) -> Element<'static, Message> {
    let trigger = Button::new("Open Basic Dialog")
        .variant(ButtonVariant::Primary)
        .on_press(Message::Dialog(DialogMessage::OpenBasic));

    if show {
        let dialog_content = container(
            column![
                // Header
                container(text("Dialog Title").size(20))
                    .padding(20)
                    .width(Fill),
                // Content
                container(
                    column![
                        text("This is a basic dialog."),
                        text("It has a title, content, and footer with action buttons."),
                    ]
                    .spacing(10)
                )
                .padding(20),
                // Footer with right-aligned buttons
                container(
                    Row::with_children([
                        Button::new("Cancel")
                            .variant(ButtonVariant::Default)
                            .on_press(Message::Dialog(DialogMessage::CloseBasic))
                            .into(),
                        Button::new("OK")
                            .variant(ButtonVariant::Primary)
                            .on_press(Message::Dialog(DialogMessage::CloseBasic))
                            .into(),
                    ])
                    .spacing(10)
                    .align_y(Alignment::Center)
                )
                .padding(20)
                .width(Fill)
                .align_x(Alignment::End)
                .class(ContainerStyleClass::DialogFooter),
            ]
            .spacing(0),
        )
        .width(Length::Fixed(500.0))
        .class(ContainerStyleClass::Card);

        Overlay::new(trigger, dialog_content)
            .backdrop_color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))
            .on_backdrop_click(Message::Dialog(DialogMessage::CloseBasic))
            .into()
    } else {
        trigger.into()
    }
}

fn custom_width_dialog(show: bool) -> Element<'static, Message> {
    let trigger = Button::new("Open Custom Width Dialog")
        .variant(ButtonVariant::Primary)
        .on_press(Message::Dialog(DialogMessage::OpenCustomWidth));

    if show {
        let dialog_content = container(
            column![
                // Header
                container(text("Custom Width Dialog").size(20))
                    .padding(20)
                    .width(Fill),
                // Content
                container(
                    column![
                        text("This dialog has a custom width."),
                        text("You can set the width using the width() method."),
                    ]
                    .spacing(10)
                )
                .padding(20),
                // Footer
                container(
                    Button::new("Close")
                        .variant(ButtonVariant::Primary)
                        .on_press(Message::Dialog(DialogMessage::CloseCustomWidth))
                )
                .padding(20)
                .width(Fill)
                .align_x(Alignment::End)
                .class(ContainerStyleClass::DialogFooter),
            ]
            .spacing(0),
        )
        .width(Fill)
        .class(ContainerStyleClass::Card);

        Overlay::new(trigger, dialog_content)
            .backdrop_color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))
            .on_backdrop_click(Message::Dialog(DialogMessage::CloseCustomWidth))
            .into()
    } else {
        trigger.into()
    }
}

fn no_header_dialog(show: bool) -> Element<'static, Message> {
    let trigger = Button::new("Open No Header Dialog")
        .variant(ButtonVariant::Primary)
        .on_press(Message::Dialog(DialogMessage::OpenNoHeader));

    if show {
        let dialog_content = container(
            column![
                // Content (no header)
                container(
                    column![
                        text("This dialog has no header.").size(20),
                        text("Content can be displayed without a header bar."),
                    ]
                    .spacing(10)
                )
                .padding(20),
                // Footer
                container(
                    Button::new("Close")
                        .variant(ButtonVariant::Primary)
                        .on_press(Message::Dialog(DialogMessage::CloseNoHeader))
                )
                .padding(20)
                .width(Fill)
                .align_x(Alignment::End)
                .class(ContainerStyleClass::DialogFooter),
            ]
            .spacing(0),
        )
        .width(Length::Fixed(500.0))
        .class(ContainerStyleClass::Card);

        Overlay::new(trigger, dialog_content)
            .backdrop_color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))
            .on_backdrop_click(Message::Dialog(DialogMessage::CloseNoHeader))
            .into()
    } else {
        trigger.into()
    }
}

fn scroll_dialog(show: bool) -> Element<'static, Message> {
    let trigger = Button::new("Open Scrolling Dialog")
        .variant(ButtonVariant::Primary)
        .on_press(Message::Dialog(DialogMessage::OpenScroll));

    if show {
        let mut content: Column<'static, Message, Theme> = column![].spacing(10);

        for i in 1..=30 {
            content = content.push(text(format!("Line {} of scrolling content", i)));
        }

        let dialog_content = container(
            column![
                // Header
                container(text("Scrolling Dialog").size(20))
                    .padding(20)
                    .width(Fill),
                // Scrollable content
                container(content).padding(20),
                // Footer
                container(
                    Button::new("Close")
                        .variant(ButtonVariant::Primary)
                        .on_press(Message::Dialog(DialogMessage::CloseScroll))
                )
                .padding(20)
                .width(Fill)
                .align_x(Alignment::End)
                .class(ContainerStyleClass::DialogFooter),
            ]
            .spacing(0),
        )
        .width(Length::Fixed(500.0))
        .class(ContainerStyleClass::Card);

        Overlay::new(trigger, dialog_content)
            .backdrop_color(Color::from_rgba(0.0, 0.0, 0.0, 0.5))
            .on_backdrop_click(Message::Dialog(DialogMessage::CloseScroll))
            .into()
    } else {
        trigger.into()
    }
}
