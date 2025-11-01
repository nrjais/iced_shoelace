use iced_widget::{Row, column, container, text};

use crate::components::{
    Divider, Menu, MenuItemType, MenuItem, MenuLabel, PopupPlacement, dropdown,
};
use crate::components::button::Button;
use crate::theme::button::ButtonVariant;
use crate::theme::container::ContainerStyleClass;
use crate::theme::sizes::SPACING;
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Dropdowns").size(32);
    let description = text(
        "Display menus attached to a trigger element. Combines positioning with menu functionality."
    )
    .size(14);

    // Basic Dropdown
    let basic_title = text("Basic Dropdown").size(24);
    let basic_desc = text("A simple dropdown with menu items").size(14);

    let basic_menu = Menu::new()
        .push(MenuItem::new("Option 1").on_select(Message::ButtonPressed("Option 1".into())))
        .push(MenuItem::new("Option 2").on_select(Message::ButtonPressed("Option 2".into())))
        .push(MenuItem::new("Option 3").on_select(Message::ButtonPressed("Option 3".into())))
        .width(200);

    let basic_dropdown = dropdown(
        Button::new("Dropdown")
            .variant(ButtonVariant::Primary)
            .suffix("▼")
            .on_press(Message::ButtonPressed("Toggle Dropdown".into())),
        basic_menu,
        true, // open
    )
    .placement(PopupPlacement::Bottom)
    .distance(4.0);

    // Dropdown with different button variants
    let variants_title = text("Button Variants").size(24);
    let variants_desc = text("Dropdowns can use any button variant").size(14);

    let menu_primary = Menu::new()
        .push(MenuItem::new("New File").on_select(Message::ButtonPressed("New".into())))
        .push(MenuItem::new("Open File").on_select(Message::ButtonPressed("Open".into())))
        .push(MenuItem::new("Save File").on_select(Message::ButtonPressed("Save".into())))
        .width(150);

    let dropdown_primary = dropdown(
        Button::new("Primary")
            .variant(ButtonVariant::Primary)
            .suffix("▼")
            .on_press(Message::ButtonPressed("Primary".into())),
        menu_primary,
        true,
    )
    .distance(4.0);

    let menu_success = Menu::new()
        .push(MenuItem::new("Confirm").on_select(Message::ButtonPressed("Confirm".into())))
        .push(MenuItem::new("Apply").on_select(Message::ButtonPressed("Apply".into())))
        .width(150);

    let dropdown_success = dropdown(
        Button::new("Success")
            .variant(ButtonVariant::Success)
            .suffix("▼")
            .on_press(Message::ButtonPressed("Success".into())),
        menu_success,
        true,
    )
    .distance(4.0);

    let menu_neutral = Menu::new()
        .push(MenuItem::new("View").on_select(Message::ButtonPressed("View".into())))
        .push(MenuItem::new("Edit").on_select(Message::ButtonPressed("Edit".into())))
        .width(150);

    let dropdown_neutral = dropdown(
        Button::new("Neutral")
            .variant(ButtonVariant::Neutral)
            .suffix("▼")
            .on_press(Message::ButtonPressed("Neutral".into())),
        menu_neutral,
        true,
    )
    .distance(4.0);

    let menu_warning = Menu::new()
        .push(MenuItem::new("Warning Action").on_select(Message::ButtonPressed("Warning".into())))
        .width(150);

    let dropdown_warning = dropdown(
        Button::new("Warning")
            .variant(ButtonVariant::Warning)
            .suffix("▼")
            .on_press(Message::ButtonPressed("Warning".into())),
        menu_warning,
        true,
    )
    .distance(4.0);

    let menu_danger = Menu::new()
        .push(MenuItem::new("Delete").on_select(Message::ButtonPressed("Delete".into())))
        .push(MenuItem::new("Remove").on_select(Message::ButtonPressed("Remove".into())))
        .width(150);

    let dropdown_danger = dropdown(
        Button::new("Danger")
            .variant(ButtonVariant::Danger)
            .suffix("▼")
            .on_press(Message::ButtonPressed("Danger".into())),
        menu_danger,
        true,
    )
    .distance(4.0);

    let variants_row = Row::with_children([
        dropdown_primary.into(),
        dropdown_success.into(),
        dropdown_neutral.into(),
        dropdown_warning.into(),
        dropdown_danger.into(),
    ])
    .spacing(15);

    // Placements
    let placements_title = text("Dropdown Placements").size(24);
    let placements_desc = text("Dropdowns support 12 placement options").size(14);

    // Top placements
    let menu_top = Menu::new()
        .push(MenuItem::new("Top Item 1"))
        .push(MenuItem::new("Top Item 2"))
        .width(120);

    let dropdown_top = dropdown(
        Button::new("Top")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("Top".into())),
        menu_top,
        true,
    )
    .placement(PopupPlacement::Top)
    .distance(4.0);

    let menu_top_start = Menu::new()
        .push(MenuItem::new("Top Start 1"))
        .push(MenuItem::new("Top Start 2"))
        .width(120);

    let dropdown_top_start = dropdown(
        Button::new("Top Start")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("TopStart".into())),
        menu_top_start,
        true,
    )
    .placement(PopupPlacement::TopStart)
    .distance(4.0);

    let menu_top_end = Menu::new()
        .push(MenuItem::new("Top End 1"))
        .push(MenuItem::new("Top End 2"))
        .width(120);

    let dropdown_top_end = dropdown(
        Button::new("Top End")
            .variant(ButtonVariant::Primary)
            .on_press(Message::ButtonPressed("TopEnd".into())),
        menu_top_end,
        true,
    )
    .placement(PopupPlacement::TopEnd)
    .distance(4.0);

    let top_row = Row::with_children([
        dropdown_top.into(),
        dropdown_top_start.into(),
        dropdown_top_end.into(),
    ])
    .spacing(15);

    // Bottom placements (most common)
    let menu_bottom = Menu::new()
        .push(MenuItem::new("Bottom Item 1"))
        .push(MenuItem::new("Bottom Item 2"))
        .width(120);

    let dropdown_bottom = dropdown(
        Button::new("Bottom")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("Bottom".into())),
        menu_bottom,
        true,
    )
    .placement(PopupPlacement::Bottom)
    .distance(4.0);

    let menu_bottom_start = Menu::new()
        .push(MenuItem::new("Bottom Start 1"))
        .push(MenuItem::new("Bottom Start 2"))
        .width(150);

    let dropdown_bottom_start = dropdown(
        Button::new("Bottom Start")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("BottomStart".into())),
        menu_bottom_start,
        true,
    )
    .placement(PopupPlacement::BottomStart)
    .distance(4.0);

    let menu_bottom_end = Menu::new()
        .push(MenuItem::new("Bottom End 1"))
        .push(MenuItem::new("Bottom End 2"))
        .width(120);

    let dropdown_bottom_end = dropdown(
        Button::new("Bottom End")
            .variant(ButtonVariant::Success)
            .on_press(Message::ButtonPressed("BottomEnd".into())),
        menu_bottom_end,
        true,
    )
    .placement(PopupPlacement::BottomEnd)
    .distance(4.0);

    let bottom_row = Row::with_children([
        dropdown_bottom.into(),
        dropdown_bottom_start.into(),
        dropdown_bottom_end.into(),
    ])
    .spacing(15);

    // Grouped menu items
    let grouped_title = text("Grouped Menu Items").size(24);
    let grouped_desc = text("Use menu labels and dividers to organize items").size(14);

    let grouped_menu = Menu::new()
        .push(MenuLabel::new("File"))
        .push(MenuItem::new("New").on_select(Message::ButtonPressed("New".into())))
        .push(MenuItem::new("Open").on_select(Message::ButtonPressed("Open".into())))
        .push(MenuItem::new("Save").on_select(Message::ButtonPressed("Save".into())))
        .push(Divider::new())
        .push(MenuLabel::new("Edit"))
        .push(MenuItem::new("Cut").on_select(Message::ButtonPressed("Cut".into())))
        .push(MenuItem::new("Copy").on_select(Message::ButtonPressed("Copy".into())))
        .push(MenuItem::new("Paste").on_select(Message::ButtonPressed("Paste".into())))
        .push(Divider::new())
        .push(MenuLabel::new("View"))
        .push(MenuItem::new("Zoom In").on_select(Message::ButtonPressed("Zoom In".into())))
        .push(MenuItem::new("Zoom Out").on_select(Message::ButtonPressed("Zoom Out".into())))
        .width(200);

    let grouped_dropdown = dropdown(
        Button::new("File Menu")
            .variant(ButtonVariant::Primary)
            .suffix("▼")
            .on_press(Message::ButtonPressed("File Menu".into())),
        grouped_menu,
        true,
    )
    .placement(PopupPlacement::BottomStart)
    .distance(4.0);

    // Menu item types
    let types_title = text("Menu Item Types").size(24);
    let types_desc = text("Menu items can be normal, checkbox, or disabled").size(14);

    let types_menu = Menu::new()
        .push(MenuItem::new("Normal Item").on_select(Message::ButtonPressed("Normal".into())))
        .push(
            MenuItem::new("Checked Item")
                .item_type(MenuItemType::Checkbox)
                .checked(true)
                .on_select(Message::ButtonPressed("Checked".into())),
        )
        .push(
            MenuItem::new("Unchecked Item")
                .item_type(MenuItemType::Checkbox)
                .checked(false)
                .on_select(Message::ButtonPressed("Unchecked".into())),
        )
        .push(Divider::new())
        .push(MenuItem::new("Disabled Item").disabled(true))
        .width(200);

    let types_dropdown = dropdown(
        Button::new("Menu Types")
            .variant(ButtonVariant::Neutral)
            .suffix("▼")
            .on_press(Message::ButtonPressed("Menu Types".into())),
        types_menu,
        true,
    )
    .placement(PopupPlacement::Bottom)
    .distance(4.0);

    // Icon menu items
    let icons_title = text("Menu Items with Icons").size(24);
    let icons_desc = text("Add prefix and suffix icons to menu items").size(14);

    let icons_menu = Menu::new()
        .push(
            MenuItem::new("New File")
                .prefix("📄")
                .suffix("Ctrl+N")
                .on_select(Message::ButtonPressed("New File".into())),
        )
        .push(
            MenuItem::new("Open Folder")
                .prefix("📁")
                .suffix("Ctrl+O")
                .on_select(Message::ButtonPressed("Open Folder".into())),
        )
        .push(
            MenuItem::new("Save")
                .prefix("💾")
                .suffix("Ctrl+S")
                .on_select(Message::ButtonPressed("Save".into())),
        )
        .push(Divider::new())
        .push(
            MenuItem::new("Settings")
                .prefix("⚙️")
                .on_select(Message::ButtonPressed("Settings".into())),
        )
        .push(
            MenuItem::new("Help")
                .prefix("❓")
                .suffix("F1")
                .on_select(Message::ButtonPressed("Help".into())),
        )
        .width(250);

    let icons_dropdown = dropdown(
        Button::new("File")
            .variant(ButtonVariant::Primary)
            .suffix("▼")
            .on_press(Message::ButtonPressed("File".into())),
        icons_menu,
        true,
    )
    .placement(PopupPlacement::BottomStart)
    .distance(4.0);

    // Distance customization
    let distance_title = text("Custom Distance").size(24);
    let distance_desc = text("Control the gap between dropdown and trigger").size(14);

    let menu_default_distance = Menu::new()
        .push(MenuItem::new("Default Distance (4px)"))
        .width(180);

    let dropdown_default_distance = dropdown(
        Button::new("Default (4px)")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Default Distance".into())),
        menu_default_distance,
        true,
    )
    .placement(PopupPlacement::Bottom);

    let menu_custom_distance = Menu::new()
        .push(MenuItem::new("Custom Distance (16px)"))
        .width(180);

    let dropdown_custom_distance = dropdown(
        Button::new("Custom (16px)")
            .variant(ButtonVariant::Neutral)
            .on_press(Message::ButtonPressed("Custom Distance".into())),
        menu_custom_distance,
        true,
    )
    .placement(PopupPlacement::Bottom)
    .distance(16.0);

    let distance_row = Row::with_children([
        dropdown_default_distance.into(),
        dropdown_custom_distance.into(),
    ])
    .spacing(15);

    // Usage notes
    let notes_title = text("Usage Notes").size(24);
    let notes = column![
        text("• Dropdowns combine a trigger element with a menu"),
        text("• Typically used with buttons as triggers"),
        text("• Supports all 12 placement options from Popup"),
        text("• Menu automatically styled with border, shadow, and padding"),
        text("• Use Menu, MenuItem, MenuLabel, and Divider for content"),
        text("• MenuItem supports normal, checkbox, and disabled states"),
        text("• Add prefix/suffix to menu items for icons and shortcuts"),
        text("• Automatically adjusts position to stay in viewport"),
        text("• Use 'open' property to control visibility (typically tied to state)"),
    ]
    .spacing(10);

    // State management example
    let state_title = text("State Management Example").size(24);
    let state_desc = text(
        "In practice, you'd toggle the 'open' state in response to button clicks"
    )
    .size(14);

    let state_code = container(
        text(
            r#"// In your update function:
Message::ToggleDropdown => {
    self.dropdown_open = !self.dropdown_open;
}

// In your view:
dropdown(
    Button::new("Options")
        .on_press(Message::ToggleDropdown),
    menu().push(menu_item("Option 1")),
    self.dropdown_open, // State controlled here
)"#,
        )
        .size(12),
    )
    .padding(SPACING.small)
    .class(ContainerStyleClass::Card);

    column![
        title,
        description,
        basic_title,
        basic_desc,
        basic_dropdown,
        variants_title,
        variants_desc,
        variants_row,
        placements_title,
        placements_desc,
        text("Top Placements").size(18),
        top_row,
        text("Bottom Placements (Most Common)").size(18),
        bottom_row,
        grouped_title,
        grouped_desc,
        grouped_dropdown,
        types_title,
        types_desc,
        types_dropdown,
        icons_title,
        icons_desc,
        icons_dropdown,
        distance_title,
        distance_desc,
        distance_row,
        state_title,
        state_desc,
        state_code,
        notes_title,
        notes,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
