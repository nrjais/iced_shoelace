use iced::alignment;
use iced_widget::{Row, column, text};

use crate::components::Badge;
use crate::components::button::Button;
use crate::components::tooltip;
use crate::theme::badge::BadgeVariant as BadgeVar;
use crate::theme::button::ButtonVariant;
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Shoelace Component Gallery").size(32);
    let subtitle = text("A comprehensive showcase of all Shoelace-inspired components").size(16);

    let description = text(
        "Navigate through the pages using the sidebar to explore each component category. \
        This gallery demonstrates buttons, badges, button groups, scrollables, and tooltips \
        with various styles and configurations.",
    )
    .size(14);

    // Quick preview of each component
    let preview_title = text("Quick Preview").size(24);

    let badge_preview = Row::with_children([
        text("Badges: ").into(),
        Badge::new("Primary").variant(BadgeVar::Primary).into(),
        Badge::new("Success").variant(BadgeVar::Success).into(),
        Badge::new("Danger").variant(BadgeVar::Danger).into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    let button_preview = Row::with_children([
        text("Buttons: ").into(),
        Button::new("Primary")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Overview Primary".into()))
            .into(),
        Button::new("Success")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("Overview Success".into()))
            .into(),
    ])
    .spacing(10);

    let tooltip_preview = Row::with_children([
        text("Tooltips: ").into(),
        tooltip(
            "Hover for tooltip!",
            Button::new("Hover me")
                .variant(ButtonVariant::Neutral)
                .on_press(Message::ButtonPressed("Tooltip Preview".into())),
        )
        .into(),
    ])
    .spacing(10);

    column![
        title,
        subtitle,
        description,
        preview_title,
        badge_preview,
        button_preview,
        tooltip_preview,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
