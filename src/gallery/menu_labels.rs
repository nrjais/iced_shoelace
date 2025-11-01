use iced::Length;
use iced_widget::{column, container, text};

use crate::components::{Divider, MenuLabel};
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Menu Labels").size(32);
    let description = text("Use menu labels to group and describe related menu items").size(14);

    // Basic menu label
    let basic_title = text("Basic Menu Label").size(24);
    let basic_desc = text("A simple menu label that describes a section").size(14);
    let basic_label = MenuLabel::new("Section Title");

    // Uppercase menu label
    let uppercase_title = text("Uppercase Menu Labels").size(24);
    let uppercase_desc = text("Use uppercase to create more prominent section headers").size(14);
    let uppercase_label = MenuLabel::new("Section Title").uppercase(true);

    // Menu example
    let menu_title = text("Menu with Labels").size(24);
    let menu_desc = text("A typical menu structure using menu labels").size(14);
    let menu_example = container(
        column![
            MenuLabel::new("Fruits"),
            text("Apple").size(14),
            text("Banana").size(14),
            text("Orange").size(14),
            Divider::new(),
            MenuLabel::new("Vegetables"),
            text("Broccoli").size(14),
            text("Carrot").size(14),
            text("Zucchini").size(14),
            Divider::new(),
            MenuLabel::new("Beverages"),
            text("Water").size(14),
            text("Juice").size(14),
            text("Tea").size(14),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(200.0));

    // Menu with uppercase labels
    let uppercase_menu_title = text("Menu with Uppercase Labels").size(24);
    let uppercase_menu_desc = text("Using uppercase for more prominent headers").size(14);
    let uppercase_menu_example = container(
        column![
            MenuLabel::new("File").uppercase(true),
            text("New").size(14),
            text("Open").size(14),
            text("Save").size(14),
            Divider::new(),
            MenuLabel::new("Edit").uppercase(true),
            text("Cut").size(14),
            text("Copy").size(14),
            text("Paste").size(14),
            Divider::new(),
            MenuLabel::new("View").uppercase(true),
            text("Zoom In").size(14),
            text("Zoom Out").size(14),
            text("Full Screen").size(14),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(200.0));

    // Sidebar navigation example
    let sidebar_title = text("Sidebar Navigation").size(24);
    let sidebar_desc = text("Use menu labels to organize navigation items").size(14);
    let sidebar_example = container(
        column![
            MenuLabel::new("Dashboard").uppercase(true),
            text("Overview").size(14),
            text("Analytics").size(14),
            text("Reports").size(14),
            Divider::new(),
            MenuLabel::new("Content").uppercase(true),
            text("Posts").size(14),
            text("Pages").size(14),
            text("Media").size(14),
            Divider::new(),
            MenuLabel::new("Settings").uppercase(true),
            text("General").size(14),
            text("Profile").size(14),
            text("Security").size(14),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(200.0));

    column![
        title,
        description,
        basic_title,
        basic_desc,
        basic_label,
        uppercase_title,
        uppercase_desc,
        uppercase_label,
        menu_title,
        menu_desc,
        menu_example,
        uppercase_menu_title,
        uppercase_menu_desc,
        uppercase_menu_example,
        sidebar_title,
        sidebar_desc,
        sidebar_example,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
