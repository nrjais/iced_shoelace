use crate::Element;
use iced::{
    Padding,
    widget::{container, text, tooltip as iced_tooltip},
};

use crate::theme::Theme;

/// Tooltip placement options matching Shoelace design system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    #[default]
    Top,
    TopStart,
    TopEnd,
    Right,
    RightStart,
    RightEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Left,
    LeftStart,
    LeftEnd,
}

/// Trigger options for showing the tooltip
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Trigger {
    #[default]
    Hover,
    Focus,
    Click,
    Manual,
}

impl From<Placement> for iced_tooltip::Position {
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::Top | Placement::TopStart | Placement::TopEnd => iced_tooltip::Position::Top,
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                iced_tooltip::Position::Bottom
            }
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                iced_tooltip::Position::Left
            }
            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                iced_tooltip::Position::Right
            }
        }
    }
}

/// A Shoelace-style tooltip component for iced using overlays
///
/// This component implements features from Shoelace's tooltip component:
/// - Multiple placements (top, right, bottom, left with start/end variations)
/// - Configurable distance from target
/// - Disabled state
/// - Uses iced's built-in overlay system for proper positioning
pub struct Tooltip<'a, Message> {
    content: String,
    child: Element<'a, Message>,
    placement: Placement,
    distance: f32,
    disabled: bool,
}

impl<'a, Message> Tooltip<'a, Message> {
    /// Creates a new tooltip with the given content and child element
    pub fn new(content: impl Into<String>, child: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            child: child.into(),
            placement: Placement::Top,
            distance: 8.0,
            disabled: false,
        }
    }

    /// Sets the tooltip placement
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the distance between tooltip and target
    pub fn distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }

    /// Sets whether the tooltip is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl<'a, Message> From<Tooltip<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(tooltip: Tooltip<'a, Message>) -> Self {
        if tooltip.disabled {
            // If disabled, just return the child
            tooltip.child
        } else {
            let theme = Theme::default();
            let tokens = theme.tokens();

            // Create tooltip with Shoelace styling
            let tooltip_text = text(tooltip.content.clone()).size(tokens.font_size.small);

            // Wrap in a container with padding
            let tooltip_container = container(tooltip_text).padding(Padding::from([
                tokens.spacing.x_small,
                tokens.spacing.small,
            ]));

            // Use iced's built-in tooltip with the styled container
            iced_tooltip(tooltip.child, tooltip_container, tooltip.placement.into())
                .gap(tooltip.distance)
                .into()
        }
    }
}

/// Convenience function to create a tooltip
pub fn tooltip<'a, Message>(
    content: impl Into<String>,
    child: impl Into<Element<'a, Message>>,
) -> Tooltip<'a, Message>
where
    Message: 'a,
{
    Tooltip::new(content, child)
}
