use iced::Length;
use iced_widget::{column, text};

use crate::components::{Divider, Menu, MenuItem, MenuItemType, MenuLabel};
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Menus").size(32);
    let description = text("Menus provide a list of options for the user to choose from").size(14);

    // Basic menu
    let basic_title = text("Basic Menu").size(24);
    let basic_desc = text("A simple menu with clickable items").size(14);
    let basic_menu = Menu::new()
        .push(MenuItem::new("Option 1").on_select(Message::MenuItemSelected))
        .push(MenuItem::new("Option 2").on_select(Message::MenuItemSelected))
        .push(MenuItem::new("Option 3").on_select(Message::MenuItemSelected))
        .width(Length::Fixed(250.0));

    // Menu with icons
    let icons_title = text("Menu with Icons").size(24);
    let icons_desc = text("Add visual indicators with prefix icons").size(14);
    let icons_menu = Menu::new()
        .push(
            MenuItem::new("New File")
                .prefix("📄")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Open Folder")
                .prefix("📁")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Save")
                .prefix("💾")
                .on_select(Message::MenuItemSelected),
        )
        .width(Length::Fixed(250.0));

    // Menu with keyboard shortcuts
    let shortcuts_title = text("Menu with Shortcuts").size(24);
    let shortcuts_desc = text("Show keyboard shortcuts in the suffix").size(14);
    let shortcuts_menu = Menu::new()
        .push(
            MenuItem::new("Copy")
                .prefix("📋")
                .suffix("⌘C")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Cut")
                .prefix("✂️")
                .suffix("⌘X")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Paste")
                .prefix("📌")
                .suffix("⌘V")
                .on_select(Message::MenuItemSelected),
        )
        .width(Length::Fixed(250.0));

    // Menu with labels and dividers
    let grouped_title = text("Grouped Menu").size(24);
    let grouped_desc = text("Organize menu items with labels and dividers").size(14);
    let grouped_menu = Menu::new()
        .push(MenuLabel::new("File Operations"))
        .push(
            MenuItem::new("New")
                .prefix("📄")
                .suffix("⌘N")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Open")
                .prefix("📁")
                .suffix("⌘O")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Save")
                .prefix("💾")
                .suffix("⌘S")
                .on_select(Message::MenuItemSelected),
        )
        .push(Divider::new())
        .push(MenuLabel::new("Edit"))
        .push(
            MenuItem::new("Cut")
                .prefix("✂️")
                .suffix("⌘X")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Copy")
                .prefix("📋")
                .suffix("⌘C")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Paste")
                .prefix("📌")
                .suffix("⌘V")
                .on_select(Message::MenuItemSelected),
        )
        .width(Length::Fixed(250.0));

    // Menu with checkbox items
    let checkbox_title = text("Menu with Checkboxes").size(24);
    let checkbox_desc = text("Use checkbox items for toggleable options").size(14);
    let checkbox_menu = Menu::new()
        .push(MenuLabel::new("View Options"))
        .push(
            MenuItem::new("Show Line Numbers")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Show Minimap")
                .item_type(MenuItemType::Checkbox)
                .checked(false)
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Word Wrap")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::MenuItemSelected),
        )
        .push(Divider::new())
        .push(MenuLabel::new("Editor"))
        .push(
            MenuItem::new("Auto Save")
                .item_type(MenuItemType::Checkbox)
                .checked(false)
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Format on Save")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::MenuItemSelected),
        )
        .width(Length::Fixed(250.0));

    // Menu with disabled items
    let disabled_title = text("Menu with Disabled Items").size(24);
    let disabled_desc = text("Show unavailable options in disabled state").size(14);
    let disabled_menu = Menu::new()
        .push(
            MenuItem::new("Available Option")
                .prefix("✓")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Disabled Option")
                .prefix("✗")
                .disabled(true)
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Another Available")
                .prefix("✓")
                .on_select(Message::MenuItemSelected),
        )
        .push(Divider::new())
        .push(
            MenuItem::new("Save")
                .prefix("💾")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Save As")
                .prefix("💾")
                .disabled(true)
                .suffix("Not available")
                .on_select(Message::MenuItemSelected),
        )
        .width(Length::Fixed(250.0));

    // Menu with loading state
    let loading_title = text("Menu with Loading State").size(24);
    let loading_desc = text("Show loading state for async operations").size(14);
    let loading_menu = Menu::new()
        .push(
            MenuItem::new("Refresh")
                .prefix("🔄")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Loading Item")
                .loading(true)
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Another Item")
                .prefix("📦")
                .on_select(Message::MenuItemSelected),
        )
        .width(Length::Fixed(250.0));

    // Complete example menu
    let complete_title = text("Complete Menu Example").size(24);
    let complete_desc =
        text("A comprehensive menu with all features combined").size(14);
    let complete_menu = Menu::new()
        .push(MenuLabel::new("File Operations").uppercase(true))
        .push(
            MenuItem::new("New Document")
                .prefix("📄")
                .suffix("⌘N")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Open File")
                .prefix("📁")
                .suffix("⌘O")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Open Recent")
                .prefix("📂")
                .suffix("⌘⇧O")
                .on_select(Message::MenuItemSelected),
        )
        .push(Divider::new())
        .push(
            MenuItem::new("Save")
                .prefix("💾")
                .suffix("⌘S")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Save As")
                .prefix("💾")
                .suffix("⌘⇧S")
                .disabled(true)
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Save All")
                .prefix("💾")
                .on_select(Message::MenuItemSelected),
        )
        .push(Divider::new())
        .push(MenuLabel::new("View Options").uppercase(true))
        .push(
            MenuItem::new("Show Sidebar")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Show Status Bar")
                .item_type(MenuItemType::Checkbox)
                .checked(false)
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Show Minimap")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::MenuItemSelected),
        )
        .push(Divider::new())
        .push(MenuLabel::new("Actions").uppercase(true))
        .push(
            MenuItem::new("Refresh")
                .prefix("🔄")
                .suffix("F5")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Settings")
                .prefix("⚙️")
                .suffix("⌘,")
                .on_select(Message::MenuItemSelected),
        )
        .push(
            MenuItem::new("Exit")
                .prefix("🚪")
                .suffix("⌘Q")
                .on_select(Message::MenuItemSelected),
        )
        .width(Length::Fixed(300.0));

    column![
        title,
        description,
        basic_title,
        basic_desc,
        basic_menu,
        icons_title,
        icons_desc,
        icons_menu,
        shortcuts_title,
        shortcuts_desc,
        shortcuts_menu,
        grouped_title,
        grouped_desc,
        grouped_menu,
        checkbox_title,
        checkbox_desc,
        checkbox_menu,
        disabled_title,
        disabled_desc,
        disabled_menu,
        loading_title,
        loading_desc,
        loading_menu,
        complete_title,
        complete_desc,
        complete_menu,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
