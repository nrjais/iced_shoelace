use crate::{
    Element,
    theme::{container::ContainerStyleClass, rule::RuleStyleClass, sizes::SPACING},
};
use iced::{
    Color, Length, Padding, Shadow,
    widget::{container, rule},
};

/// A Shoelace-style divider component for iced
///
/// This component implements all the features from Shoelace's divider component:
/// - Horizontal (default) or vertical orientation
/// - Customizable width (thickness)
/// - Customizable color
/// - Customizable spacing around the divider
/// - Used to visually separate or group elements
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::Divider;
///
/// // Simple horizontal divider
/// let divider = Divider::new();
///
/// // Vertical divider with custom width
/// let divider = Divider::new()
///     .vertical(true)
///     .width(2.0);
///
/// // Custom color and spacing
/// let divider = Divider::new()
///     .color(Color::from_rgb(1.0, 0.0, 0.0))
///     .spacing(32.0);
/// ```
pub struct Divider {
    vertical: bool,
    width: f32,
    color: Option<Color>,
    spacing: f32,
}

impl Divider {
    /// Creates a new horizontal divider with default settings
    ///
    /// Defaults:
    /// - Horizontal orientation
    /// - Width: 1px (matching Shoelace's default)
    /// - Color: neutral-300 (from theme)
    /// - Spacing: 1rem (matching Shoelace's default)
    pub fn new() -> Self {
        Self {
            vertical: false,
            width: 1.0,
            color: None,
            spacing: SPACING.x_small, // 1rem / 16px
        }
    }

    /// Sets whether the divider is vertical
    ///
    /// When vertical is true, the divider draws from top to bottom instead of left to right.
    /// The parent container should use flex layout (row) for vertical dividers to display properly.
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }

    /// Sets the thickness of the divider line
    ///
    /// In Shoelace, this corresponds to the `--width` CSS custom property.
    /// Default is 1px.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Sets a custom color for the divider
    ///
    /// In Shoelace, this corresponds to the `--color` CSS custom property.
    /// If not set, the theme's neutral-300 color will be used.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the spacing around the divider
    ///
    /// In Shoelace, this corresponds to the `--spacing` CSS custom property.
    /// Default is 1rem (16px).
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new()
    }
}

/// Creates a new horizontal divider with default settings
///
/// This is a convenience function equivalent to `Divider::new()`.
///
/// ## Example
///
/// ```rust
/// use iced_shoelace::components::divider;
///
/// let divider = divider();
/// ```
pub fn divider() -> Divider {
    Divider::new()
}

impl<'a, Message: 'a> From<Divider> for Element<'a, Message> {
    fn from(divider: Divider) -> Self {
        // Use iced's built-in Rule widget for the divider line
        // Note: for vertical rules, the parameter is width; for horizontal rules, it's height
        let rule_widget = if divider.vertical {
            rule::vertical(divider.width)
        } else {
            rule::horizontal(divider.width)
        };

        // Apply custom styling based on color
        let styled_rule = if let Some(color) = divider.color {
            rule_widget.class(RuleStyleClass::Custom(color))
        } else {
            // Use default theme styling
            rule_widget.class(RuleStyleClass::Default)
        };

        // Wrap in a container to add spacing with transparent background (matching Shoelace)
        let padding = if divider.vertical {
            Padding::from([0.0, divider.spacing])
        } else {
            Padding::from([divider.spacing, 0.0])
        };

        let transparent_style = ContainerStyleClass::Custom {
            background: None,
            text_color: None,
            border_color: None,
            border_width: 0.0,
            border_radius: 0.0,
            shadow: Shadow::default(),
            snap: false,
        };

        container(styled_rule)
            .padding(padding)
            .width(if divider.vertical {
                Length::Shrink
            } else {
                Length::Fill
            })
            .height(if divider.vertical {
                Length::Fill
            } else {
                Length::Shrink
            })
            .class(transparent_style)
            .into()
    }
}
