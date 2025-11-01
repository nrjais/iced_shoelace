use crate::{
    Element,
    theme::{
        pallete::{ColorToken, ColorValue, ColorVariant},
        sizes::{FONT_SIZE, LINE_HEIGHT, SPACING},
        text::TextStyleClass,
    },
};
use iced::{
    Alignment, Font, Padding,
    widget::{container, text},
};
use iced_core::Length;

/// A Shoelace-style menu label component for iced
///
/// This component implements the features from Shoelace's menu-label component:
/// - Non-interactive heading for grouping menu items
/// - Styled with small font size and semibold weight
/// - Muted color for de-emphasized appearance
/// - Used to provide context for related menu items
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::MenuLabel;
///
/// // Simple menu label
/// let label = MenuLabel::new("Fruits");
///
/// // Menu label with custom styling
/// let label = MenuLabel::new("Vegetables")
///     .uppercase(true);
/// ```
pub struct MenuLabel {
    content: String,
    uppercase: bool,
}

impl MenuLabel {
    /// Creates a new menu label with the given content
    ///
    /// The label will be styled according to Shoelace's design system:
    /// - Small font size
    /// - Semibold font weight
    /// - Muted neutral color
    /// - Appropriate padding for menu context
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            uppercase: false,
        }
    }

    /// Sets whether the label text should be displayed in uppercase
    ///
    /// When true, the text will be transformed to uppercase.
    /// This is useful for creating more prominent section headers.
    pub fn uppercase(mut self, uppercase: bool) -> Self {
        self.uppercase = uppercase;
        self
    }

    /// Gets the appropriate padding for the menu label
    /// Following Shoelace spec: minimal vertical, small horizontal padding
    fn get_padding(&self) -> Padding {
        // Menu labels in Shoelace use: var(--sl-spacing-2x-small) var(--sl-spacing-small)
        // Which is approximately 0.125rem 0.5rem (2px 8px)
        Padding::from([SPACING.x2_small, SPACING.small])
    }
}

impl Default for MenuLabel {
    fn default() -> Self {
        Self::new("")
    }
}

/// Creates a new menu label with the given content
///
/// This is a convenience function equivalent to `MenuLabel::new(content)`.
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::menu_label;
///
/// let label = menu_label("Section Title");
/// ```
pub fn menu_label(content: impl Into<String>) -> MenuLabel {
    MenuLabel::new(content)
}

impl<'a, Message: 'a> From<MenuLabel> for Element<'a, Message> {
    fn from(label: MenuLabel) -> Self {
        let padding = label.get_padding();

        // Transform content to uppercase if requested
        let content = if label.uppercase {
            label.content.to_uppercase()
        } else {
            label.content
        };

        // Menu label text styling - x-small size (following Shoelace spec), semibold weight
        let label_text = text(content)
            .size(FONT_SIZE.x_small)
            .font(Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            })
            .line_height(LINE_HEIGHT.dense)
            // Use neutral-600 for muted appearance
            .class(TextStyleClass {
                color: Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C600)),
            });

        // Build the container with appropriate styling
        container(label_text)
            .padding(padding)
            .align_x(Alignment::Start)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }
}
