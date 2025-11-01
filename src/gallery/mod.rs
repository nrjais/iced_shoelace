use iced::{Length, alignment};
use iced_widget::{Row, column, container, text};

use crate::components::Badge;
use crate::components::button::Button;
use crate::components::button_group::button_group_with;
use crate::components::scrollable;
use crate::components::{TooltipPlacement, tooltip};
use crate::theme::badge::BadgeVariant as BadgeVar;
use crate::theme::button::{ButtonSize, ButtonVariant};
use crate::theme::{ScrollableClass, Theme};
use crate::{Element, Message, Page};

pub fn view(current_page: Page, button_count: usize) -> Element<'static, Message> {
    let content = Row::new()
        .push(navigation_sidebar(current_page))
        .push(page_content(current_page, button_count));

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

fn page_content(page: Page, button_count: usize) -> Element<'static, Message> {
    let content = match page {
        Page::Overview => overview_page(button_count),
        Page::Badges => badges_page(),
        Page::Buttons => buttons_page(),
        Page::ButtonGroups => button_groups_page(),
        Page::Scrollables => scrollables_page(),
        Page::Tooltips => tooltips_page(),
    };

    scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn overview_page(button_count: usize) -> Element<'static, Message> {
    let title = text("Shoelace Component Gallery").size(32);
    let subtitle = text("A comprehensive showcase of all Shoelace-inspired components").size(16);

    let description = text(
        "Navigate through the pages using the sidebar to explore each component category. \
        This gallery demonstrates buttons, badges, button groups, scrollables, and tooltips \
        with various styles and configurations.",
    )
    .size(14);

    let counter = text(format!("Total button clicks: {}", button_count)).size(18);

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
        counter,
        preview_title,
        badge_preview,
        button_preview,
        tooltip_preview,
    ]
    .spacing(20)
    .padding(20)
    .into()
}

fn badges_page() -> Element<'static, Message> {
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

fn buttons_page() -> Element<'static, Message> {
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

fn button_groups_page() -> Element<'static, Message> {
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

fn scrollables_page() -> Element<'static, Message> {
    let title = text("Scrollable Styles").size(32);
    let description = text("Different scrollable styles using Shoelace tokens").size(14);

    // Default scrollable
    let default_scrollable = {
        let content = column![
            text("Default Scrollable").size(16),
            text("Visible rail with medium scroller").size(12),
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
            .height(150)
            .width(300)
    };

    // Subtle scrollable
    let subtle_scrollable = {
        let content = column![
            text("Subtle Scrollable").size(16),
            text("Rail appears on hover").size(12),
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
            .height(150)
            .width(300)
    };

    // Comparison row
    let comparison_title = text("Side by Side Comparison").size(24);
    let scrollable_row =
        Row::with_children([default_scrollable.into(), subtle_scrollable.into()]).spacing(40);

    // Usage notes
    let notes_title = text("Usage Notes").size(24);
    let notes = column![
        text("• Default scrollable: Best for content areas where scrolling is expected"),
        text("• Subtle scrollable: Best for minimal UI where scrollbars should be unobtrusive"),
        text("• Both styles support custom width and height"),
        text("• Scrollbars automatically appear when content overflows"),
    ]
    .spacing(10);

    column![
        title,
        description,
        comparison_title,
        scrollable_row,
        notes_title,
        notes,
    ]
    .spacing(20)
    .padding(20)
    .into()
}

fn tooltips_page() -> Element<'static, Message> {
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
