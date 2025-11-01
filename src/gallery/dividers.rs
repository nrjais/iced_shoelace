use iced::{Color, Length, alignment};
use iced_widget::{Row, column, text};

use crate::components::Divider;
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Dividers").size(32);
    let description = text("Use dividers to visually separate or group elements").size(14);

    // Basic divider
    let basic_title = text("Basic Divider").size(24);
    let basic_desc = text("A simple horizontal divider").size(14);
    let basic_divider = Divider::new();

    // Width variations
    let width_title = text("Width Variations").size(24);
    let width_desc = text("Customize the thickness of the divider").size(14);
    let width_1px = Divider::new().width(1.0);
    let width_2px = Divider::new().width(2.0);
    let width_4px = Divider::new().width(4.0);

    // Color variations
    let color_title = text("Color Variations").size(24);
    let color_desc = text("Customize the color of the divider").size(14);
    let color_default = Divider::new();
    let color_red = Divider::new().color(Color::from_rgb(0.9, 0.2, 0.2));
    let color_blue = Divider::new().color(Color::from_rgb(0.2, 0.5, 0.9));
    let color_green = Divider::new().color(Color::from_rgb(0.2, 0.7, 0.3));

    // Spacing variations
    let spacing_title = text("Spacing Variations").size(24);
    let spacing_desc = text("Customize the space around the divider").size(14);
    let spacing_small = Divider::new().spacing(8.0);
    let spacing_default = Divider::new(); // 16px default
    let spacing_large = Divider::new().spacing(32.0);

    // Vertical dividers
    let vertical_title = text("Vertical Dividers").size(24);
    let vertical_desc = text("Dividers can also be displayed vertically").size(14);
    let vertical_example = Row::with_children([
        text("First").into(),
        Divider::new().vertical(true).into(),
        text("Second").into(),
        Divider::new().vertical(true).into(),
        text("Third").into(),
    ])
    .spacing(0)
    .align_y(alignment::Vertical::Center)
    .height(Length::Fixed(40.0));

    // Custom vertical divider with styling
    let vertical_styled = Row::with_children([
        text("Item 1").into(),
        Divider::new()
            .vertical(true)
            .width(2.0)
            .color(Color::from_rgb(0.2, 0.5, 0.9))
            .spacing(16.0)
            .into(),
        text("Item 2").into(),
        Divider::new()
            .vertical(true)
            .width(2.0)
            .color(Color::from_rgb(0.2, 0.5, 0.9))
            .spacing(16.0)
            .into(),
        text("Item 3").into(),
    ])
    .spacing(0)
    .align_y(alignment::Vertical::Center)
    .height(Length::Fixed(40.0));

    // Usage in menus/lists
    let menu_title = text("Menu Dividers").size(24);
    let menu_desc = text("Use dividers to group menu items").size(14);
    let menu_example = column![
        text("File"),
        text("Edit"),
        text("View"),
        Divider::new(),
        text("Settings"),
        text("Help"),
        Divider::new(),
        text("Exit"),
    ]
    .spacing(8)
    .padding(16)
    .width(Length::Fixed(200.0));

    column![
        title,
        description,
        basic_title,
        basic_desc,
        text("Above"),
        basic_divider,
        text("Below"),
        width_title,
        width_desc,
        text("1px (default)"),
        width_1px,
        text("2px"),
        width_2px,
        text("4px"),
        width_4px,
        color_title,
        color_desc,
        text("Default (neutral)"),
        color_default,
        text("Red"),
        color_red,
        text("Blue"),
        color_blue,
        text("Green"),
        color_green,
        spacing_title,
        spacing_desc,
        text("Small spacing (8px)"),
        spacing_small,
        text("Default spacing (16px)"),
        spacing_default,
        text("Large spacing (32px)"),
        spacing_large,
        vertical_title,
        vertical_desc,
        vertical_example,
        text("Styled vertical dividers"),
        vertical_styled,
        menu_title,
        menu_desc,
        menu_example,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
