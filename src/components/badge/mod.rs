use crate::{
    Element,
    theme::{
        badge::BadgeVariant,
        container::ContainerStyleClass,
        sizes::{BORDER_RADIUS, FONT_SIZE, LINE_HEIGHT, SPACING},
    },
};
use iced::{
    Alignment, Padding,
    widget::{container, text},
};
use iced_core::Length;

/// A Shoelace-style badge component for iced
///
/// This component implements all the features from Shoelace's badge component:
/// - Multiple variants (primary, success, neutral, warning, danger)
/// - Pill style (fully rounded corners)
/// - Pulse animation effect
pub struct Badge {
    content: String,
    variant: BadgeVariant,
    pill: bool,
    pulse: bool,
}

impl Badge {
    /// Creates a new badge with the given content
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            variant: BadgeVariant::Primary,
            pill: false,
            pulse: false,
        }
    }

    /// Sets the badge variant
    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets whether the badge has fully rounded corners (pill style)
    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }

    /// Sets whether the badge pulsates (for drawing attention)
    pub fn pulse(mut self, pulse: bool) -> Self {
        self.pulse = pulse;
        self
    }

    /// Gets the appropriate padding for the badge
    /// Following Shoelace spec: small padding for compact appearance
    fn get_padding(&self) -> Padding {
        // Badges in Shoelace use: 0 0.5rem (0px 8px)
        Padding::from([SPACING.x2_small, SPACING.small])
    }

    /// Gets the appropriate border radius
    fn get_border_radius(&self) -> f32 {
        if self.pill {
            // Pill badges should be fully rounded
            BORDER_RADIUS.x_large * 10.0
        } else {
            // Standard badges use medium border radius
            BORDER_RADIUS.medium
        }
    }
}

impl<'a, Message: 'a> From<Badge> for Element<'a, Message> {
    fn from(badge: Badge) -> Self {
        let padding = badge.get_padding();
        let border_radius = badge.get_border_radius();

        // Badge text styling - small size
        let badge_text = text(badge.content)
            .size(FONT_SIZE.small)
            .line_height(LINE_HEIGHT.dense);

        // Create style class using ContainerStyleClass
        let style_class = ContainerStyleClass::Badge {
            variant: badge.variant,
            border_radius,
            pulse: badge.pulse,
        };

        // Build the container with appropriate styling
        container(badge_text)
            .padding(padding)
            .class(style_class)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into()
    }
}
