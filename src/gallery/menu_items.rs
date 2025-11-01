use iced::Length;
use iced_widget::{column, container, text};

use crate::components::{Divider, MenuItem, MenuItemType, MenuLabel};
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Menu Items").size(32);
    let description = text("Menu items are clickable elements that can be used in menus and dropdowns").size(14);

    // Basic menu items
    let basic_title = text("Basic Menu Items").size(24);
    let basic_desc = text("Simple clickable menu items").size(14);
    let basic_menu = container(
        column![
            MenuItem::new("Option 1").on_select(Message::MenuItemSelected),
            MenuItem::new("Option 2").on_select(Message::MenuItemSelected),
            MenuItem::new("Option 3").on_select(Message::MenuItemSelected),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(250.0));

    // Menu items with prefix icons
    let prefix_title = text("Menu Items with Prefix").size(24);
    let prefix_desc = text("Add icons or text before the label").size(14);
    let prefix_menu = container(
        column![
            MenuItem::new("New File")
                .prefix("📄")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Open Folder")
                .prefix("📁")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Save")
                .prefix("💾")
                .on_select(Message::MenuItemSelected),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(250.0));

    // Menu items with suffix content
    let suffix_title = text("Menu Items with Suffix").size(24);
    let suffix_desc = text("Show keyboard shortcuts or additional info").size(14);
    let suffix_menu = container(
        column![
            MenuItem::new("Copy")
                .prefix("📋")
                .suffix("⌘C")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Cut")
                .prefix("✂️")
                .suffix("⌘X")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Paste")
                .prefix("📌")
                .suffix("⌘V")
                .on_select(Message::MenuItemSelected),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(250.0));

    // Disabled menu items
    let disabled_title = text("Disabled Menu Items").size(24);
    let disabled_desc = text("Use disabled state for unavailable options").size(14);
    let disabled_menu = container(
        column![
            MenuItem::new("Available Option")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Disabled Option")
                .disabled(true)
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Another Available")
                .on_select(Message::MenuItemSelected),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(250.0));

    // Loading menu items
    let loading_title = text("Loading State").size(24);
    let loading_desc = text("Show loading state for async operations").size(14);
    let loading_menu = container(
        column![
            MenuItem::new("Normal Item")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Loading Item")
                .loading(true)
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Another Normal Item")
                .on_select(Message::MenuItemSelected),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(250.0));

    // Checkbox menu items
    let checkbox_title = text("Checkbox Menu Items").size(24);
    let checkbox_desc = text("Use checkbox items for toggleable options").size(14);
    let checkbox_menu = container(
        column![
            MenuItem::new("Show Line Numbers")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Show Minimap")
                .item_type(MenuItemType::Checkbox)
                .checked(false)
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Word Wrap")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::MenuItemSelected),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(250.0));

    // Complete menu example with labels and dividers
    let complete_title = text("Complete Menu Example").size(24);
    let complete_desc = text("A full menu with labels, dividers, and various item types").size(14);
    let complete_menu = container(
        column![
            MenuLabel::new("File Operations"),
            MenuItem::new("New")
                .prefix("📄")
                .suffix("⌘N")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Open")
                .prefix("📁")
                .suffix("⌘O")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Save")
                .prefix("💾")
                .suffix("⌘S")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Save As")
                .prefix("💾")
                .disabled(true)
                .on_select(Message::MenuItemSelected),
            Divider::new(),
            MenuLabel::new("View Options"),
            MenuItem::new("Show Sidebar")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Show Status Bar")
                .item_type(MenuItemType::Checkbox)
                .checked(false)
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Full Screen")
                .item_type(MenuItemType::Checkbox)
                .checked(false)
                .on_select(Message::MenuItemSelected),
            Divider::new(),
            MenuLabel::new("Actions"),
            MenuItem::new("Refresh")
                .prefix("🔄")
                .on_select(Message::MenuItemSelected),
            MenuItem::new("Settings")
                .prefix("⚙️")
                .on_select(Message::MenuItemSelected),
        ]
        .spacing(4)
    )
    .padding(8)
    .width(Length::Fixed(300.0));

    column![
        title,
        description,
        basic_title,
        basic_desc,
        basic_menu,
        prefix_title,
        prefix_desc,
        prefix_menu,
        suffix_title,
        suffix_desc,
        suffix_menu,
        disabled_title,
        disabled_desc,
        disabled_menu,
        loading_title,
        loading_desc,
        loading_menu,
        checkbox_title,
        checkbox_desc,
        checkbox_menu,
        complete_title,
        complete_desc,
        complete_menu,
    ]
    .spacing(20)
    .padding(20)
    .into()
}

