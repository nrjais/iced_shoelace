use iced_widget::{column, text};

use crate::components::button::Button;
use crate::components::button_group::button_group_with;
use crate::theme::button::ButtonVariant;
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Button Groups").size(32);
    let description = text("Group related buttons together").size(14);

    // Basic button group
    let basic_title = text("Basic Button Group").size(24);
    let basic_group = button_group_with(vec![
        Button::new("Left")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Group Left".into())),
        Button::new("Middle")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Group Middle".into())),
        Button::new("Right")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Group Right".into())),
    ]);

    // Button group with label
    let labeled_title = text("Button Group with Label").size(24);
    let labeled_group = button_group_with(vec![
        Button::new("Bold")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Bold".into())),
        Button::new("Italic")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Italic".into())),
        Button::new("Underline")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Underline".into())),
    ])
    .label("Text Formatting");

    // Button group with different variants
    let action_title = text("Mixed Variants").size(24);
    let action_group = button_group_with(vec![
        Button::new("View")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("View".into())),
        Button::new("Edit")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("Edit".into())),
        Button::new("Delete")
            .variant(ButtonVariant::Danger)
            .on_press(Message::ButtonPressed("Delete Group".into())),
    ])
    .label("Actions");

    // Button group with outline buttons
    let outline_title = text("Outline Button Group").size(24);
    let outline_group = button_group_with(vec![
        Button::new("One")
            .variant(ButtonVariant::Primary)
            .outline(true)
            .on_press(Message::ButtonPressed("One".into())),
        Button::new("Two")
            .variant(ButtonVariant::Primary)
            .outline(true)
            .on_press(Message::ButtonPressed("Two".into())),
        Button::new("Three")
            .variant(ButtonVariant::Primary)
            .outline(true)
            .on_press(Message::ButtonPressed("Three".into())),
    ]);

    // Toolbar example
    let toolbar_title = text("Toolbar Example").size(24);
    let toolbar_group = button_group_with(vec![
        Button::new("New")
            .variant(ButtonVariant::Success)
            .prefix("➕")
            .on_press(Message::ButtonPressed("New".into())),
        Button::new("Open")
            .variant(ButtonVariant::Primary)
            .prefix("📂")
            .on_press(Message::ButtonPressed("Open".into())),
        Button::new("Save")
            .variant(ButtonVariant::Primary)
            .prefix("💾")
            .on_press(Message::ButtonPressed("Save".into())),
    ])
    .label("File Operations");

    column![
        title,
        description,
        basic_title,
        basic_group,
        labeled_title,
        labeled_group,
        action_title,
        action_group,
        outline_title,
        outline_group,
        toolbar_title,
        toolbar_group,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
