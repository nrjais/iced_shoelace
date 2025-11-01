use iced::alignment;
use iced_widget::{Row, column, text};

use crate::components::button::Button;
use crate::theme::button::{ButtonSize, ButtonVariant};
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Buttons").size(32);
    let description = text("Interactive button components with various styles").size(14);

    // Variants
    let variants_title = text("Variants").size(24);
    let variants_row = Row::with_children([
        Button::new("Default")
            .variant(ButtonVariant::Default)
            .on_press(Message::ButtonPressed("Default".into()))
            .into(),
        Button::new("Primary")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Primary".into()))
            .into(),
        Button::new("Success")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("Success".into()))
            .into(),
        Button::new("Neutral")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Neutral".into()))
            .into(),
        Button::new("Warning")
            .variant(ButtonVariant::Warning)
            .on_press(Message::ButtonPressed("Warning".into()))
            .into(),
        Button::new("Danger")
            .variant(ButtonVariant::Danger)
            .on_press(Message::ButtonPressed("Danger".into()))
            .into(),
    ])
    .spacing(10);

    // Sizes
    let sizes_title = text("Sizes").size(24);
    let sizes_row = Row::with_children([
        Button::new("Small")
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Small)
            .on_press(Message::ButtonPressed("Small".into()))
            .into(),
        Button::new("Medium")
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Medium)
            .on_press(Message::ButtonPressed("Medium".into()))
            .into(),
        Button::new("Large")
            .variant(ButtonVariant::Primary)
            .size(ButtonSize::Large)
            .on_press(Message::ButtonPressed("Large".into()))
            .into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    // Outline
    let outline_title = text("Outline Buttons").size(24);
    let outline_row = Row::with_children([
        Button::new("Default")
            .variant(ButtonVariant::Default)
            .outline(true)
            .on_press(Message::ButtonPressed("Outline Default".into()))
            .into(),
        Button::new("Primary")
            .variant(ButtonVariant::Primary)
            .outline(true)
            .on_press(Message::ButtonPressed("Outline Primary".into()))
            .into(),
        Button::new("Success")
            .variant(ButtonVariant::Success)
            .outline(true)
            .on_press(Message::ButtonPressed("Outline Success".into()))
            .into(),
        Button::new("Danger")
            .variant(ButtonVariant::Danger)
            .outline(true)
            .on_press(Message::ButtonPressed("Outline Danger".into()))
            .into(),
    ])
    .spacing(10);

    // Pill
    let pill_title = text("Pill Buttons").size(24);
    let pill_row = Row::with_children([
        Button::new("Default")
            .variant(ButtonVariant::Default)
            .pill(true)
            .on_press(Message::ButtonPressed("Pill Default".into()))
            .into(),
        Button::new("Primary")
            .variant(ButtonVariant::Primary)
            .pill(true)
            .on_press(Message::ButtonPressed("Pill Primary".into()))
            .into(),
        Button::new("Success")
            .variant(ButtonVariant::Success)
            .pill(true)
            .on_press(Message::ButtonPressed("Pill Success".into()))
            .into(),
    ])
    .spacing(10);

    // Text buttons
    let text_title = text("Text Buttons").size(24);
    let text_row = Row::with_children([
        Button::new("Text Default")
            .variant(ButtonVariant::Text)
            .on_press(Message::ButtonPressed("Text Default".into()))
            .into(),
        Button::new("Text Primary")
            .variant(ButtonVariant::Text)
            .on_press(Message::ButtonPressed("Text Primary".into()))
            .into(),
    ])
    .spacing(10);

    // States
    let states_title = text("States").size(24);
    let states_row = Row::with_children([
        Button::new("Normal")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Normal".into()))
            .into(),
        Button::new("Loading")
            .variant(ButtonVariant::Primary)
            .loading(true)
            .on_press(Message::ButtonPressed("Loading".into()))
            .into(),
        Button::new("Disabled")
            .variant(ButtonVariant::Primary)
            .disabled(true)
            .on_press(Message::ButtonPressed("Disabled".into()))
            .into(),
    ])
    .spacing(10);

    // Prefix and Suffix
    let prefix_suffix_title = text("Prefix & Suffix").size(24);
    let prefix_suffix_row = Row::with_children([
        Button::new("Settings")
            .variant(ButtonVariant::Primary)
            .prefix("⚙")
            .on_press(Message::ButtonPressed("Settings".into()))
            .into(),
        Button::new("Download")
            .variant(ButtonVariant::Success)
            .suffix("↓")
            .on_press(Message::ButtonPressed("Download".into()))
            .into(),
        Button::new("Delete")
            .variant(ButtonVariant::Danger)
            .prefix("🗑")
            .suffix("×")
            .on_press(Message::ButtonPressed("Delete".into()))
            .into(),
    ])
    .spacing(10);

    column![
        title,
        description,
        variants_title,
        variants_row,
        sizes_title,
        sizes_row,
        outline_title,
        outline_row,
        pill_title,
        pill_row,
        text_title,
        text_row,
        states_title,
        states_row,
        prefix_suffix_title,
        prefix_suffix_row,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
