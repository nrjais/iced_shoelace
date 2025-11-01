use iced::alignment;
use iced_widget::{Row, column, container, text};

use crate::components::{PopupPlacement, popup};
use crate::components::button::Button;
use crate::theme::button::ButtonVariant;
use crate::theme::container::ContainerStyleClass;
use crate::theme::sizes::SPACING;
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Popups").size(32);
    let description = text(
        "Display floating content relative to an anchor element. Unlike tooltips, popups require manual control."
    )
    .size(14);

    // Basic Popups
    let basic_title = text("Basic Popup").size(24);
    let basic_desc = text("A simple popup anchored to a button").size(14);

    let basic_popup = popup(
        Button::new("Anchor")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Basic Popup".into())),
        container(
            column![
                text("Popup Content").size(16),
                text("This is a popup positioned relative to the anchor").size(12),
            ]
            .spacing(8)
            .padding(SPACING.small)
        ),
        true, // active
    )
    .placement(PopupPlacement::Bottom)
    .distance(8.0);

    // Placements
    let placements_title = text("Popup Placements").size(24);
    let placements_desc =
        text("Popups support 12 placement options for precise positioning").size(14);

    // Top placements
    let popup_top = popup(
        Button::new("Top")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Top".into())),
        container(text("Top")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::Top)
    .distance(8.0);

    let popup_top_start = popup(
        Button::new("Top Start")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("TopStart".into())),
        container(text("Top Start")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::TopStart)
    .distance(8.0);

    let popup_top_end = popup(
        Button::new("Top End")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("TopEnd".into())),
        container(text("Top End")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::TopEnd)
    .distance(8.0);

    let top_row = Row::with_children([
        popup_top.into(),
        popup_top_start.into(),
        popup_top_end.into(),
    ])
    .spacing(15);

    // Bottom placements
    let popup_bottom = popup(
        Button::new("Bottom")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("Bottom".into())),
        container(text("Bottom")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::Bottom)
    .distance(8.0);

    let popup_bottom_start = popup(
        Button::new("Bottom Start")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("BottomStart".into())),
        container(text("Bottom Start")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::BottomStart)
    .distance(8.0);

    let popup_bottom_end = popup(
        Button::new("Bottom End")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("BottomEnd".into())),
        container(text("Bottom End")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::BottomEnd)
    .distance(8.0);

    let bottom_row = Row::with_children([
        popup_bottom.into(),
        popup_bottom_start.into(),
        popup_bottom_end.into(),
    ])
    .spacing(15);

    // Left placements
    let popup_left = popup(
        Button::new("Left")
            .variant(ButtonVariant::Warning)
            .on_press(Message::ButtonPressed("Left".into())),
        container(text("Left")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::Left)
    .distance(8.0);

    let popup_left_start = popup(
        Button::new("Left Start")
            .variant(ButtonVariant::Warning)
            .on_press(Message::ButtonPressed("LeftStart".into())),
        container(text("Left Start")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::LeftStart)
    .distance(8.0);

    let popup_left_end = popup(
        Button::new("Left End")
            .variant(ButtonVariant::Warning)
            .on_press(Message::ButtonPressed("LeftEnd".into())),
        container(text("Left End")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::LeftEnd)
    .distance(8.0);

    let left_row = Row::with_children([
        popup_left.into(),
        popup_left_start.into(),
        popup_left_end.into(),
    ])
    .spacing(15);

    // Right placements
    let popup_right = popup(
        Button::new("Right")
            .variant(ButtonVariant::Danger)
            .on_press(Message::ButtonPressed("Right".into())),
        container(text("Right")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::Right)
    .distance(8.0);

    let popup_right_start = popup(
        Button::new("Right Start")
            .variant(ButtonVariant::Danger)
            .on_press(Message::ButtonPressed("RightStart".into())),
        container(text("Right Start")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::RightStart)
    .distance(8.0);

    let popup_right_end = popup(
        Button::new("Right End")
            .variant(ButtonVariant::Danger)
            .on_press(Message::ButtonPressed("RightEnd".into())),
        container(text("Right End")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::RightEnd)
    .distance(8.0);

    let right_row = Row::with_children([
        popup_right.into(),
        popup_right_start.into(),
        popup_right_end.into(),
    ])
    .spacing(15);

    // Custom distance
    let distance_title = text("Custom Distance").size(24);
    let distance_desc = text("Control the gap between popup and anchor").size(14);

    let popup_default_distance = popup(
        Button::new("Default (0px)")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Default Distance".into())),
        container(text("Default distance")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::Bottom);

    let popup_custom_distance = popup(
        Button::new("20px Distance")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Custom Distance".into())),
        container(text("20px distance")).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::Bottom)
    .distance(20.0);

    let distance_row = Row::with_children([
        popup_default_distance.into(),
        popup_custom_distance.into(),
    ])
    .spacing(15);

    // Styled popups
    let styled_title = text("Styled Popups").size(24);
    let styled_desc = text("Popups can use different container styles").size(14);

    let popup_card_style = popup(
        Button::new("Card Style")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Card Style".into())),
        container(
            column![
                text("Card-style Popup").size(14),
                text("White background, border, shadow").size(11),
            ]
            .spacing(5)
            .padding(SPACING.small)
        ),
        true,
    )
    .placement(PopupPlacement::Bottom)
    .distance(8.0)
    .style(ContainerStyleClass::Card);

    let popup_tooltip_style = popup(
        Button::new("Tooltip Style")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("Tooltip Style".into())),
        container(text("Dark tooltip style").size(11)).padding(SPACING.small),
        true,
    )
    .placement(PopupPlacement::Bottom)
    .distance(8.0)
    .style(ContainerStyleClass::Tooltip);

    let styled_row = Row::with_children([popup_card_style.into(), popup_tooltip_style.into()])
        .spacing(15);

    // Rich content
    let rich_title = text("Rich Content").size(24);
    let rich_desc = text("Popups can contain any content including interactive elements").size(14);

    let rich_content = column![
        text("User Menu").size(16),
        text("john.doe@example.com").size(11),
        Button::new("Profile")
            .variant(ButtonVariant::Default)
            .on_press(Message::ButtonPressed("Profile".into())),
        Button::new("Settings")
            .variant(ButtonVariant::Default)
            .on_press(Message::ButtonPressed("Settings".into())),
        Button::new("Logout")
            .variant(ButtonVariant::Danger)
            .on_press(Message::ButtonPressed("Logout".into())),
    ]
    .spacing(8)
    .padding(SPACING.small)
    .align_x(alignment::Horizontal::Center);

    let popup_rich = popup(
        Button::new("User Menu")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("User Menu".into())),
        container(rich_content),
        true,
    )
    .placement(PopupPlacement::BottomEnd)
    .distance(8.0);

    // Usage notes
    let notes_title = text("Usage Notes").size(24);
    let notes = column![
        text("• Popups require manual control via the 'active' property"),
        text("• Use for dropdowns, context menus, and interactive overlays"),
        text("• 12 placement options for precise positioning"),
        text("• Automatically adjusts position to stay in viewport (when shift is enabled)"),
        text("• Can contain any content including interactive elements"),
        text("• For automatic hover display, use Tooltip instead"),
    ]
    .spacing(10);

    column![
        title,
        description,
        basic_title,
        basic_desc,
        basic_popup,
        placements_title,
        placements_desc,
        text("Top Placements").size(18),
        top_row,
        text("Bottom Placements").size(18),
        bottom_row,
        text("Left Placements").size(18),
        left_row,
        text("Right Placements").size(18),
        right_row,
        distance_title,
        distance_desc,
        distance_row,
        styled_title,
        styled_desc,
        styled_row,
        rich_title,
        rich_desc,
        popup_rich,
        notes_title,
        notes,
    ]
    .spacing(20)
    .padding(20)
    .into()
}

