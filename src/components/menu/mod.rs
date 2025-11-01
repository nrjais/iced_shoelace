use crate::{
    Element,
    theme::{
        container::ContainerStyleClass,
        pallete::{ColorToken, ColorValue, ColorVariant},
        sizes::{BORDER_RADIUS, SPACING},
    },
};
use iced::{
    Color, Length, Shadow,
    widget::{column, container},
};

/// A Shoelace-style menu component for iced
///
/// This component implements the features from Shoelace's menu component:
/// - Container for menu items and menu labels
/// - Proper styling with border, shadow, and padding
/// - Configurable width
/// - Supports dividers for grouping
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::{Menu, MenuItem, MenuLabel, Divider};
///
/// // Simple menu
/// let menu = Menu::new()
///     .push(MenuItem::new("Option 1").on_select(Message::Option1))
///     .push(MenuItem::new("Option 2").on_select(Message::Option2));
///
/// // Menu with labels and dividers
/// let menu = Menu::new()
///     .push(MenuLabel::new("File"))
///     .push(MenuItem::new("New").on_select(Message::New))
///     .push(MenuItem::new("Open").on_select(Message::Open))
///     .push(Divider::new())
///     .push(MenuLabel::new("Edit"))
///     .push(MenuItem::new("Cut").on_select(Message::Cut))
///     .push(MenuItem::new("Copy").on_select(Message::Copy));
/// ```
pub struct Menu<'a, Message> {
    children: Vec<Element<'a, Message>>,
    width: Length,
    height: Length,
    padding: f32,
    spacing: f32,
}

impl<'a, Message> Menu<'a, Message>
where
    Message: Clone + 'a,
{
    /// Creates a new empty menu
    ///
    /// The menu will be styled according to Shoelace's design system:
    /// - White background (neutral-0)
    /// - Border with neutral-200 color
    /// - Small shadow for elevation
    /// - Medium border radius
    /// - Appropriate padding and spacing
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            width: Length::Shrink,
            height: Length::Shrink,
            padding: SPACING.x_small,
            spacing: SPACING.x3_small,
        }
    }

    /// Adds a child element to the menu
    ///
    /// This can be a MenuItem, MenuLabel, Divider, or any other element.
    pub fn push(mut self, child: impl Into<Element<'a, Message>>) -> Self {
        self.children.push(child.into());
        self
    }

    /// Sets the width of the menu
    ///
    /// Default is Length::Shrink to fit the content.
    /// Use Length::Fixed(width) for a specific width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the menu
    ///
    /// Default is Length::Shrink to fit the content.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the padding around the menu content
    ///
    /// Default is SPACING.x_small (4px).
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the spacing between menu items
    ///
    /// Default is SPACING.x3_small (1px).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl<'a, Message> Default for Menu<'a, Message>
where
    Message: Clone + 'a,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'a> From<Menu<'a, Message>> for Element<'a, Message>
where
    Message: Clone,
{
    fn from(menu: Menu<'a, Message>) -> Self {
        // Create a column with all the menu children
        let mut menu_column = column([])
            .spacing(menu.spacing)
            .width(Length::Fill)
            .height(Length::Shrink);

        // Add all children to the column
        for child in menu.children {
            menu_column = menu_column.push(child);
        }

        // Create custom menu styling using ColorToken for theme-aware colors
        // Following Shoelace design: neutral-0 background, neutral-700 text, neutral-200 border
        let menu_style = ContainerStyleClass::Custom {
            background: Some(ColorToken::new(ColorVariant::NeutralBase, ColorValue::C50)),
            text_color: Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C700)),
            border_color: Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C200)),
            border_width: 1.0,
            border_radius: BORDER_RADIUS.medium,
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
            snap: false,
        };

        // Wrap in a container with menu styling
        container(menu_column)
            .padding(menu.padding)
            .width(menu.width)
            .height(menu.height)
            .class(menu_style)
            .into()
    }
}

/// Creates a new menu
///
/// This is a convenience function equivalent to `Menu::new()`.
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::{menu, menu_item};
///
/// let my_menu = menu()
///     .push(menu_item("Settings").on_select(Message::OpenSettings))
///     .push(menu_item("About").on_select(Message::ShowAbout));
/// ```
pub fn menu<'a, Message>() -> Menu<'a, Message>
where
    Message: Clone + 'a,
{
    Menu::new()
}

/// Creates a new menu with initial children
///
/// This is a convenience function for creating a menu with multiple items at once.
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::{menu_with, menu_item};
///
/// let my_menu = menu_with(vec![
///     menu_item("Settings").on_select(Message::OpenSettings).into(),
///     menu_item("About").on_select(Message::ShowAbout).into(),
/// ]);
/// ```
pub fn menu_with<'a, Message>(children: Vec<Element<'a, Message>>) -> Menu<'a, Message>
where
    Message: Clone + 'a,
{
    let mut menu = Menu::new();
    for child in children {
        menu = menu.push(child);
    }
    menu
}
