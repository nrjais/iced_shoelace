use iced::alignment;
use iced_widget::{Row, column, text};

use crate::components::Badge;
use crate::theme::badge::BadgeVariant as BadgeVar;
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Badges").size(32);
    let description = text("Use badges to highlight important information").size(14);

    // Badge variants
    let variants_title = text("Variants").size(24);
    let badge_variants_row = Row::with_children([
        Badge::new("Primary").variant(BadgeVar::Primary).into(),
        Badge::new("Success").variant(BadgeVar::Success).into(),
        Badge::new("Neutral").variant(BadgeVar::Neutral).into(),
        Badge::new("Warning").variant(BadgeVar::Warning).into(),
        Badge::new("Danger").variant(BadgeVar::Danger).into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    // Pill badges
    let pill_title = text("Pill Badges").size(24);
    let pill_desc = text("Rounded pill-shaped badges").size(14);
    let pill_badges_row = Row::with_children([
        Badge::new("Primary")
            .variant(BadgeVar::Primary)
            .pill(true)
            .into(),
        Badge::new("Success")
            .variant(BadgeVar::Success)
            .pill(true)
            .into(),
        Badge::new("Neutral")
            .variant(BadgeVar::Neutral)
            .pill(true)
            .into(),
        Badge::new("Warning")
            .variant(BadgeVar::Warning)
            .pill(true)
            .into(),
        Badge::new("Danger")
            .variant(BadgeVar::Danger)
            .pill(true)
            .into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    // Pulse badges
    let pulse_title = text("Pulse Badges (Animated)").size(24);
    let pulse_desc = text("Badges with pulse animation for attention-grabbing").size(14);
    let pulse_badges_row = Row::with_children([
        Badge::new("1")
            .variant(BadgeVar::Primary)
            .pulse(true)
            .into(),
        Badge::new("New")
            .variant(BadgeVar::Success)
            .pulse(true)
            .into(),
        Badge::new("!").variant(BadgeVar::Danger).pulse(true).into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    // Usage examples
    let usage_title = text("Usage Examples").size(24);
    let usage_desc = text("Badges can be used in various contexts").size(14);

    let notification_example = Row::with_children([
        text("Notifications").into(),
        Badge::new("3")
            .variant(BadgeVar::Danger)
            .pill(true)
            .pulse(true)
            .into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    let status_example = Row::with_children([
        text("Status:").into(),
        Badge::new("Online").variant(BadgeVar::Success).into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    let category_example = Row::with_children([
        Badge::new("Rust")
            .variant(BadgeVar::Neutral)
            .pill(true)
            .into(),
        Badge::new("UI")
            .variant(BadgeVar::Primary)
            .pill(true)
            .into(),
        Badge::new("Desktop")
            .variant(BadgeVar::Success)
            .pill(true)
            .into(),
    ])
    .spacing(10)
    .align_y(alignment::Vertical::Center);

    column![
        title,
        description,
        variants_title,
        badge_variants_row,
        pill_title,
        pill_desc,
        pill_badges_row,
        pulse_title,
        pulse_desc,
        pulse_badges_row,
        usage_title,
        usage_desc,
        notification_example,
        status_example,
        category_example,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
