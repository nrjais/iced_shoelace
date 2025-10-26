use iced::{Length, alignment};
use iced_widget::{Row, column, container, text};

use crate::components::button::{Button, Size, Variant};
use crate::{Element, Message};

pub fn view(button_count: usize) -> Element<'static, Message> {
    let title = text("Shoelace Button Gallery").size(32).width(Length::Fill);

    // Variants section
    let variants_title = text("Variants").size(24);
    let variants_row = Row::with_children([
        Button::new("Default")
            .variant(Variant::Default)
            .on_press(Message::ButtonPressed("Default".into()))
            .into(),
        Button::new("Primary")
            .variant(Variant::Primary)
            .on_press(Message::ButtonPressed("Primary".into()))
            .into(),
        Button::new("Success")
            .variant(Variant::Success)
            .on_press(Message::ButtonPressed("Success".into()))
            .into(),
        Button::new("Neutral")
            .variant(Variant::Neutral)
            .on_press(Message::ButtonPressed("Neutral".into()))
            .into(),
        Button::new("Warning")
            .variant(Variant::Warning)
            .on_press(Message::ButtonPressed("Warning".into()))
            .into(),
        Button::new("Danger")
            .variant(Variant::Danger)
            .on_press(Message::ButtonPressed("Danger".into()))
            .into(),
    ])
    .spacing(10);

    // Sizes section
    let sizes_title = text("Sizes").size(24);
    let sizes_row = Row::with_children([
        Button::new("Small")
            .variant(Variant::Primary)
            .size(Size::Small)
            .on_press(Message::ButtonPressed("Small".into()))
            .into(),
        Button::new("Medium")
            .variant(Variant::Primary)
            .size(Size::Medium)
            .on_press(Message::ButtonPressed("Medium".into()))
            .into(),
        Button::new("Large")
            .variant(Variant::Primary)
            .size(Size::Large)
            .on_press(Message::ButtonPressed("Large".into()))
            .into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    // Outline buttons
    let outline_title = text("Outline").size(24);
    let outline_row = Row::with_children([
        Button::new("Default")
            .variant(Variant::Default)
            .outline(true)
            .on_press(Message::ButtonPressed("Outline Default".into()))
            .into(),
        Button::new("Primary")
            .variant(Variant::Primary)
            .outline(true)
            .on_press(Message::ButtonPressed("Outline Primary".into()))
            .into(),
        Button::new("Success")
            .variant(Variant::Success)
            .outline(true)
            .on_press(Message::ButtonPressed("Outline Success".into()))
            .into(),
        Button::new("Danger")
            .variant(Variant::Danger)
            .outline(true)
            .on_press(Message::ButtonPressed("Outline Danger".into()))
            .into(),
    ])
    .spacing(10);

    // Pill buttons
    let pill_title = text("Pill").size(24);
    let pill_row = Row::with_children([
        Button::new("Default")
            .variant(Variant::Default)
            .pill(true)
            .on_press(Message::ButtonPressed("Pill Default".into()))
            .into(),
        Button::new("Primary")
            .variant(Variant::Primary)
            .pill(true)
            .on_press(Message::ButtonPressed("Pill Primary".into()))
            .into(),
        Button::new("Success")
            .variant(Variant::Success)
            .pill(true)
            .on_press(Message::ButtonPressed("Pill Success".into()))
            .into(),
    ])
    .spacing(10);

    // Circle buttons
    let circle_title = text("Circle").size(24);
    let circle_row = Row::with_children([
        Button::new("X")
            .variant(Variant::Default)
            .circle(true)
            .on_press(Message::ButtonPressed("Circle Default".into()))
            .into(),
        Button::new("✓")
            .variant(Variant::Success)
            .circle(true)
            .on_press(Message::ButtonPressed("Circle Success".into()))
            .into(),
        Button::new("✕")
            .variant(Variant::Danger)
            .circle(true)
            .on_press(Message::ButtonPressed("Circle Danger".into()))
            .into(),
    ])
    .spacing(10);

    // Text buttons
    let text_title = text("Text Buttons").size(24);
    let text_row = Row::with_children([
        Button::new("Text Default")
            .variant(Variant::Text)
            .on_press(Message::ButtonPressed("Text Default".into()))
            .into(),
        Button::new("Text Primary")
            .variant(Variant::Text)
            .on_press(Message::ButtonPressed("Text Primary".into()))
            .into(),
    ])
    .spacing(10);

    // States section
    let states_title = text("States").size(24);
    let states_row = Row::with_children([
        Button::new("Loading")
            .variant(Variant::Primary)
            .loading(true)
            .on_press(Message::ButtonPressed("Loading".into()))
            .into(),
        Button::new("Disabled").variant(Variant::Primary).into(),
        Button::new("With Caret")
            .variant(Variant::Primary)
            .caret(true)
            .on_press(Message::ButtonPressed("Caret".into()))
            .into(),
    ])
    .spacing(10);

    // Click counter
    let counter = text(format!("Total button clicks: {}", button_count)).size(18);

    let content = column![
        title,
        variants_title,
        variants_row,
        sizes_title,
        sizes_row,
        outline_title,
        outline_row,
        pill_title,
        pill_row,
        circle_title,
        circle_row,
        text_title,
        text_row,
        states_title,
        states_row,
        counter,
    ]
    .spacing(20)
    .padding(30);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
