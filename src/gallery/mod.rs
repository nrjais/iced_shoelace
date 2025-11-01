use iced::{Length, alignment};
use iced_widget::{Row, column, text};

use crate::components::button::{Button, Size, Variant};
use crate::components::button_group::button_group_with;
use crate::components::scrollable;
use crate::components::{TooltipPlacement, tooltip};
use crate::theme::{ScrollableClass, Theme};
use crate::{Element, Message};

pub fn view(button_count: usize) -> Element<'static, Message> {
    let title = text("Shoelace Button Gallery").size(32).width(Length::Fill);

    let theme_dark = Button::new("Dark Theme").on_press(Message::SwitchTheme(Theme::Dark));
    let theme_light = Button::new("Light Theme").on_press(Message::SwitchTheme(Theme::Light));

    let theme_row = Row::with_children([theme_dark.into(), theme_light.into()]).spacing(10);
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
        Button::new("Disabled")
            .variant(Variant::Primary)
            .disabled(true)
            .on_press(Message::ButtonPressed("Disabled".into()))
            .into(),
    ])
    .spacing(10);

    // Prefix and Suffix section
    let prefix_suffix_title = text("Prefix & Suffix").size(24);
    let prefix_suffix_row = Row::with_children([
        Button::new("Settings")
            .variant(Variant::Primary)
            .prefix("⚙")
            .on_press(Message::ButtonPressed("Settings".into()))
            .into(),
        Button::new("Download")
            .variant(Variant::Success)
            .suffix("↓")
            .on_press(Message::ButtonPressed("Download".into()))
            .into(),
        Button::new("Delete")
            .variant(Variant::Danger)
            .prefix("🗑")
            .suffix("×")
            .on_press(Message::ButtonPressed("Delete".into()))
            .into(),
    ])
    .spacing(10);

    // Button Groups section
    let button_groups_title = text("Button Groups").size(24);
    let button_groups_desc = text("Group related buttons together").size(14);

    // Basic button group
    let basic_group = button_group_with(vec![
        Button::new("Left")
            .variant(Variant::Primary)
            .on_press(Message::ButtonPressed("Group Left".into())),
        Button::new("Middle")
            .variant(Variant::Primary)
            .on_press(Message::ButtonPressed("Group Middle".into())),
        Button::new("Right")
            .variant(Variant::Primary)
            .on_press(Message::ButtonPressed("Group Right".into())),
    ]);

    // Button group with label
    let labeled_group = button_group_with(vec![
        Button::new("Bold")
            .variant(Variant::Neutral)
            .on_press(Message::ButtonPressed("Bold".into())),
        Button::new("Italic")
            .variant(Variant::Neutral)
            .on_press(Message::ButtonPressed("Italic".into())),
        Button::new("Underline")
            .variant(Variant::Neutral)
            .on_press(Message::ButtonPressed("Underline".into())),
    ])
    .label("Text Formatting");

    // Button group with different variants
    let action_group = button_group_with(vec![
        Button::new("View")
            .variant(Variant::Primary)
            .on_press(Message::ButtonPressed("View".into())),
        Button::new("Edit")
            .variant(Variant::Success)
            .on_press(Message::ButtonPressed("Edit".into())),
        Button::new("Delete")
            .variant(Variant::Danger)
            .on_press(Message::ButtonPressed("Delete Group".into())),
    ])
    .label("Actions");

    // Button group with outline buttons
    let outline_group = button_group_with(vec![
        Button::new("One")
            .variant(Variant::Primary)
            .outline(true)
            .on_press(Message::ButtonPressed("One".into())),
        Button::new("Two")
            .variant(Variant::Primary)
            .outline(true)
            .on_press(Message::ButtonPressed("Two".into())),
        Button::new("Three")
            .variant(Variant::Primary)
            .outline(true)
            .on_press(Message::ButtonPressed("Three".into())),
    ]);

    let button_groups_row =
        Row::with_children([basic_group.into(), labeled_group.into()]).spacing(20);

    let button_groups_row2 =
        Row::with_children([action_group.into(), outline_group.into()]).spacing(20);

    // Disabled variants section
    let disabled_title = text("Disabled Variants").size(24);
    let disabled_row = Row::with_children([
        Button::new("Primary")
            .variant(Variant::Primary)
            .disabled(true)
            .into(),
        Button::new("Success")
            .variant(Variant::Success)
            .disabled(true)
            .into(),
        Button::new("Danger")
            .variant(Variant::Danger)
            .disabled(true)
            .into(),
        Button::new("Outline")
            .variant(Variant::Primary)
            .outline(true)
            .disabled(true)
            .into(),
        Button::new("Text")
            .variant(Variant::Text)
            .disabled(true)
            .into(),
    ])
    .spacing(10);

    // Scrollable styles section
    let scrollable_title = text("Scrollable Styles").size(24);
    let scrollable_desc =
        text("This demo shows different scrollable styles using Shoelace tokens").size(14);

    // Create small scrollable areas to demonstrate styles
    let default_scrollable = {
        let content = column![
            text("Default Scrollable").size(16),
            text("Visible rail with medium scroller"),
            text("Line 1"),
            text("Line 2"),
            text("Line 3"),
            text("Line 4"),
            text("Line 5"),
            text("Line 6"),
            text("Line 7"),
            text("Line 8"),
        ]
        .spacing(5)
        .padding(10);

        scrollable(content)
            .class(ScrollableClass::Default)
            .height(120)
            .width(200)
    };

    let subtle_scrollable = {
        let content = column![
            text("Subtle Scrollable").size(16),
            text("Rail appears on hover"),
            text("Line 1"),
            text("Line 2"),
            text("Line 3"),
            text("Line 4"),
            text("Line 5"),
            text("Line 6"),
            text("Line 7"),
            text("Line 8"),
        ]
        .spacing(5)
        .padding(10);

        scrollable(content)
            .class(ScrollableClass::Subtle)
            .height(120)
            .width(200)
    };

    let scrollable_row =
        Row::with_children([default_scrollable.into(), subtle_scrollable.into()]).spacing(20);

    // Tooltips section
    let tooltips_title = text("Tooltips").size(24);
    let tooltips_desc = text("Hover over the elements to see tooltips").size(14);

    let tooltip_button_top = tooltip(
        "This is a tooltip on top!",
        Button::new("Hover me (Top)")
            .variant(Variant::Primary)
            .on_press(Message::ButtonPressed("Tooltip Top".into())),
    )
    .placement(TooltipPlacement::Top);

    let tooltip_button_bottom = tooltip(
        "This is a tooltip on bottom",
        Button::new("Hover me (Bottom)")
            .variant(Variant::Success)
            .on_press(Message::ButtonPressed("Tooltip Bottom".into())),
    )
    .placement(TooltipPlacement::Bottom);

    let tooltip_button_left = tooltip(
        "This tooltip appears on the left",
        Button::new("Hover me (Left)")
            .variant(Variant::Warning)
            .on_press(Message::ButtonPressed("Tooltip Left".into())),
    )
    .placement(TooltipPlacement::Left);

    let tooltip_button_right = tooltip(
        "This tooltip appears on the right",
        Button::new("Hover me (Right)")
            .variant(Variant::Danger)
            .on_press(Message::ButtonPressed("Tooltip Right".into())),
    )
    .placement(TooltipPlacement::Right);

    let tooltip_text = tooltip(
        "Even text can have tooltips!",
        text("Hover over this text").size(16),
    );

    let tooltip_custom_distance = tooltip(
        "This tooltip is further away (distance: 20px)",
        Button::new("Custom Distance")
            .variant(Variant::Neutral)
            .on_press(Message::ButtonPressed("Custom Distance".into())),
    )
    .placement(TooltipPlacement::Top)
    .distance(20.0);

    let tooltips_row = Row::with_children([
        tooltip_button_top.into(),
        tooltip_button_bottom.into(),
        tooltip_button_left.into(),
        tooltip_button_right.into(),
        tooltip_custom_distance.into(),
    ])
    .spacing(10);

    // Click counter
    let counter = text(format!("Total button clicks: {}", button_count)).size(18);

    let content = column![
        title,
        theme_row,
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
        button_groups_title,
        button_groups_desc,
        button_groups_row,
        button_groups_row2,
        disabled_title,
        disabled_row,
        scrollable_title,
        scrollable_desc,
        scrollable_row,
        tooltips_title,
        tooltips_desc,
        tooltips_row,
        tooltip_text,
        counter,
    ]
    .spacing(20)
    .padding(8);

    scrollable(content).into()
}
