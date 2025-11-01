use iced_widget::{Row, column, text};

use crate::components::Badge;
use crate::components::button::Button;
use crate::components::{TooltipPlacement, tooltip};
use crate::theme::badge::BadgeVariant as BadgeVar;
use crate::theme::button::ButtonVariant;
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Tooltips").size(32);
    let description =
        text("Hover over the elements to see tooltips in different positions").size(14);

    // Placements
    let placements_title = text("Tooltip Placements").size(24);
    let placements_desc = text("Tooltips can be positioned on all four sides").size(14);

    let tooltip_top = tooltip(
        "This is a tooltip on top!",
        Button::new("Top")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Tooltip Top".into())),
    )
    .placement(TooltipPlacement::Top);

    let tooltip_bottom = tooltip(
        "This is a tooltip on bottom",
        Button::new("Bottom")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("Tooltip Bottom".into())),
    )
    .placement(TooltipPlacement::Bottom);

    let tooltip_left = tooltip(
        "This tooltip appears on the left",
        Button::new("Left")
            .variant(ButtonVariant::Warning)
            .on_press(Message::ButtonPressed("Tooltip Left".into())),
    )
    .placement(TooltipPlacement::Left);

    let tooltip_right = tooltip(
        "This tooltip appears on the right",
        Button::new("Right")
            .variant(ButtonVariant::Danger)
            .on_press(Message::ButtonPressed("Tooltip Right".into())),
    )
    .placement(TooltipPlacement::Right);

    let placements_row = Row::with_children([
        tooltip_top.into(),
        tooltip_bottom.into(),
        tooltip_left.into(),
        tooltip_right.into(),
    ])
    .spacing(15);

    // Custom distance
    let distance_title = text("Custom Distance").size(24);
    let distance_desc = text("Control the distance between tooltip and element").size(14);

    let tooltip_default_distance = tooltip(
        "Default distance",
        Button::new("Default")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Default Distance".into())),
    )
    .placement(TooltipPlacement::Top);

    let tooltip_custom_distance = tooltip(
        "Custom distance (20px)",
        Button::new("20px Distance")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Custom Distance".into())),
    )
    .placement(TooltipPlacement::Top)
    .distance(20.0);

    let distance_row = Row::with_children([
        tooltip_default_distance.into(),
        tooltip_custom_distance.into(),
    ])
    .spacing(15);

    // Tooltip on different elements
    let elements_title = text("Tooltips on Different Elements").size(24);
    let elements_desc = text("Tooltips work with any element").size(14);

    let tooltip_text = tooltip(
        "Even text can have tooltips!",
        text("Hover over this text").size(16),
    );

    let tooltip_badge = tooltip(
        "This is a badge with a tooltip",
        Badge::new("Badge").variant(BadgeVar::Primary),
    );

    let elements_row = Row::with_children([tooltip_text.into(), tooltip_badge.into()]).spacing(15);

    // Usage notes
    let notes_title = text("Usage Notes").size(24);
    let notes = column![
        text("• Tooltips provide contextual information on hover"),
        text("• Choose placement based on available space"),
        text("• Keep tooltip text concise and informative"),
        text("• Default distance is 8px, customizable as needed"),
    ]
    .spacing(10);

    column![
        title,
        description,
        placements_title,
        placements_desc,
        placements_row,
        distance_title,
        distance_desc,
        distance_row,
        elements_title,
        elements_desc,
        elements_row,
        notes_title,
        notes,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
