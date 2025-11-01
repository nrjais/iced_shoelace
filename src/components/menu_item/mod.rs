use crate::{
    Element,
    theme::{
        Theme,
        container::ContainerStyleClass,
        pallete::{ColorToken, ColorValue, ColorVariant},
        sizes::{BORDER_RADIUS, FONT_SIZE, LINE_HEIGHT, SPACING},
        text::TextStyleClass,
    },
};
use iced::{
    Alignment, Border, Color, Padding, Shadow,
    border::Radius,
    widget::{Component, component, container, text},
};
use iced_core::Length;
use iced_widget::Row;

/// Type of menu item
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuItemType {
    /// Normal menu item (default)
    #[default]
    Normal,
    /// Checkbox menu item (shows checkmark when selected)
    Checkbox,
}

/// A Shoelace-style menu item component for iced
///
/// This component implements the features from Shoelace's menu-item component:
/// - Clickable menu items with hover and focus states
/// - Support for checked state (checkbox-style menu items)
/// - Disabled state
/// - Prefix and suffix content (icons, text)
/// - Loading state
/// - Value property for identification
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::MenuItem;
///
/// // Simple menu item
/// let item = MenuItem::new("Option 1")
///     .on_select(Message::Selected);
///
/// // Checkbox menu item
/// let item = MenuItem::new("Enable feature")
///     .item_type(MenuItemType::Checkbox)
///     .checked(true)
///     .on_select(Message::ToggleFeature);
///
/// // Menu item with prefix icon
/// let item = MenuItem::new("Save")
///     .prefix("💾")
///     .on_select(Message::Save);
/// ```
pub struct MenuItem<Message> {
    label: String,
    value: Option<String>,
    item_type: MenuItemType,
    checked: bool,
    disabled: bool,
    loading: bool,
    prefix: Option<String>,
    suffix: Option<String>,
    on_select: Option<Message>,
}

impl<Message> MenuItem<Message> {
    /// Creates a new menu item with the given label
    ///
    /// The item will be styled according to Shoelace's design system:
    /// - Medium font size
    /// - Appropriate padding for comfortable clicking
    /// - Hover and focus states with subtle background color
    /// - Smooth transitions
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: None,
            item_type: MenuItemType::Normal,
            checked: false,
            disabled: false,
            loading: false,
            prefix: None,
            suffix: None,
            on_select: None,
        }
    }

    /// Sets the value property for identifying this menu item
    ///
    /// This can be used to distinguish between menu items when handling selection events.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Sets the type of menu item (normal or checkbox)
    pub fn item_type(mut self, item_type: MenuItemType) -> Self {
        self.item_type = item_type;
        self
    }

    /// Sets whether the menu item is checked (for checkbox-style items)
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets whether the menu item is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the menu item is in a loading state
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets the prefix content (displayed before the label)
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Sets the suffix content (displayed after the label)
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Sets the message that will be produced when the menu item is selected
    pub fn on_select(mut self, message: Message) -> Self {
        self.on_select = Some(message);
        self
    }

    /// Gets the appropriate padding for the menu item
    /// Following Shoelace spec: var(--sl-spacing-2x-small) var(--sl-spacing-small)
    fn get_padding(&self) -> Padding {
        // Menu items in Shoelace use: 0.125rem 0.75rem (approximately 2px 12px)
        Padding::from([SPACING.x2_small, SPACING.small])
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Selected,
    Hovered(bool),
}

impl<'a, Message> Component<'a, Message, Theme> for MenuItem<Message>
where
    Message: Clone,
{
    type State = MenuItemState;
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Selected => {
                if !self.loading && !self.disabled {
                    // Toggle checked state for checkbox items
                    if self.item_type == MenuItemType::Checkbox {
                        self.checked = !self.checked;
                    }
                    self.on_select.clone()
                } else {
                    None
                }
            }
            Event::Hovered(is_hovered) => {
                state.is_hovered = is_hovered;
                None
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<'a, Self::Event> {
        let padding = self.get_padding();
        let font_size = FONT_SIZE.medium;
        let line_height = LINE_HEIGHT.dense;

        // Build label text
        let label_str = if self.loading {
            "⋯".to_string()
        } else {
            self.label.clone()
        };

        let is_hovered = state.is_hovered;
        let disabled = self.disabled;

        // Build the menu item content with optional prefix, checkmark, label, and suffix
        let mut row_content = Row::new()
            .spacing(SPACING.small)
            .align_y(Alignment::Center);

        // Add checkmark for checkbox items when checked
        if self.item_type == MenuItemType::Checkbox {
            let check_text = if self.checked { "✓" } else { "" };
            row_content = row_content.push(
                text(check_text)
                    .size(font_size)
                    .line_height(line_height)
                    .class(TextStyleClass {
                        color: Some(ColorToken::new(ColorVariant::Blue, ColorValue::C600)),
                    }),
            );
        }

        // Add prefix if present
        if let Some(prefix_text) = &self.prefix
            && !prefix_text.is_empty()
        {
            row_content = row_content.push(
                text(prefix_text.clone())
                    .size(font_size)
                    .line_height(line_height)
                    .class(TextStyleClass {
                        color: if disabled {
                            Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C400))
                        } else {
                            Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C700))
                        },
                    }),
            );
        }

        // Add main label
        row_content = row_content.push(
            text(label_str)
                .size(font_size)
                .line_height(line_height)
                .class(TextStyleClass {
                    color: if disabled {
                        Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C400))
                    } else {
                        Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C700))
                    },
                }),
        );

        // Add suffix if present
        if let Some(suffix_text) = &self.suffix
            && !suffix_text.is_empty()
        {
            row_content = row_content.push(
                text(suffix_text.clone())
                    .size(font_size)
                    .line_height(line_height)
                    .class(TextStyleClass {
                        color: if disabled {
                            Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C400))
                        } else {
                            Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C500))
                        },
                    }),
            );
        }

        let content: Element<'a, Event> = row_content.into();

        // Determine background color based on state
        let background_color = if disabled {
            None
        } else if is_hovered {
            // Use a light neutral background on hover (neutral-50)
            Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C50))
        } else {
            None
        };

        // Build the container with appropriate styling
        let has_on_select = self.on_select.is_some();
        let container_style = ContainerStyleClass::Custom {
            background: background_color,
            text_color: None,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(BORDER_RADIUS.small),
            },
            shadow: Shadow::default(),
            snap: false,
        };

        let container = container(content)
            .padding(padding)
            .width(Length::Fill)
            .height(Length::Shrink)
            .class(container_style);

        // Wrap in a mouse area to handle hover and click events
        let mouse_area = iced::widget::mouse_area(container)
            .on_enter(Event::Hovered(true))
            .on_exit(Event::Hovered(false));

        let mouse_area = if !disabled && !self.loading && has_on_select {
            mouse_area.on_press(Event::Selected)
        } else {
            mouse_area
        };

        mouse_area.into()
    }
}

#[derive(Debug, Clone, Default)]
pub struct MenuItemState {
    is_hovered: bool,
}

impl<'a, Message> From<MenuItem<Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(menu_item: MenuItem<Message>) -> Self {
        component(menu_item)
    }
}

/// Creates a new menu item with the given label
///
/// This is a convenience function equivalent to `MenuItem::new(label)`.
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::menu_item;
///
/// let item = menu_item("Settings")
///     .on_select(Message::OpenSettings);
/// ```
pub fn menu_item<Message>(label: impl Into<String>) -> MenuItem<Message> {
    MenuItem::new(label)
}
