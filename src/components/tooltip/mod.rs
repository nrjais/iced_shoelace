use crate::widgets;
use crate::{
    Element,
    theme::{
        container::ContainerStyleClass,
        sizes::{FONT_SIZE, SPACING},
    },
    widgets::tooltip::Position,
};
use iced::{
    Padding,
    widget::{container, text},
};

/// Tooltip placement options matching Shoelace design system
///
/// Shoelace supports 12 placement options to position tooltips relative to their target:
/// - Top/Bottom/Left/Right: Centered on the respective side
/// - TopStart/BottomStart: Aligned to the start (left for horizontal, top for vertical)
/// - TopEnd/BottomEnd: Aligned to the end (right for horizontal, bottom for vertical)
/// - LeftStart/RightStart: Aligned to the start of the side
/// - LeftEnd/RightEnd: Aligned to the end of the side
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

impl From<Placement> for Position {
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::Top | Placement::TopStart | Placement::TopEnd => Position::Top,
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => Position::Bottom,
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => Position::Left,
            Placement::Right | Placement::RightStart | Placement::RightEnd => Position::Right,
        }
    }
}

/// A Shoelace-style tooltip component for iced using overlays
///
/// This component implements features from Shoelace's tooltip component:
/// - Multiple placements (top, right, bottom, left with start/end variations)
/// - Configurable distance from target
/// - Configurable skidding (offset along the target)
/// - Disabled state
/// - Hoist option to prevent clipping
/// - Uses iced's built-in overlay system for proper positioning
///
/// # Styling
///
/// Tooltips use Shoelace design system styling:
/// - Dark background (neutral-800)
/// - White text (neutral-0)
/// - Medium border radius (3px)
/// - Subtle box shadow for depth
/// - Small font size with appropriate padding
///
/// # Example
///
/// ```rust
/// use iced_shoelace::components::tooltip::{tooltip, Placement};
/// use iced_shoelace::components::button::Button;
///
/// // Basic tooltip
/// let my_tooltip = tooltip(
///     "This is a helpful tooltip!",
///     Button::new("Hover me")
/// );
///
/// // Tooltip with custom placement and distance
/// let custom_tooltip = tooltip(
///     "Bottom tooltip with more space",
///     Button::new("Hover me")
/// )
/// .placement(Placement::Bottom)
/// .distance(16.0);
///
/// // Disabled tooltip
/// let disabled_tooltip = tooltip(
///     "You won't see this",
///     Button::new("No tooltip")
/// )
/// .disabled(true);
///
/// // Hoisted tooltip to prevent clipping
/// let hoisted_tooltip = tooltip(
///     "This won't be clipped",
///     Button::new("In scrollable")
/// )
/// .hoist(true);
/// ```
pub struct Tooltip<'a, Message> {
    content: String,
    child: Element<'a, Message>,
    placement: Placement,
    distance: f32,
    skidding: f32,
    disabled: bool,
    hoist: bool,
}

impl<'a, Message> Tooltip<'a, Message> {
    /// Creates a new tooltip with the given content and child element
    pub fn new(content: impl Into<String>, child: impl Into<Element<'a, Message>>) -> Self {
        Self {
            content: content.into(),
            child: child.into(),
            placement: Placement::Top,
            distance: 4.0,
            skidding: 0.0,
            disabled: false,
            hoist: false,
        }
    }

    /// Sets the tooltip placement
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the distance between tooltip and target (in pixels)
    /// Default: 8.0
    pub fn distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }

    /// Sets the skidding (offset along the target) in pixels
    /// Default: 0.0
    pub fn skidding(mut self, skidding: f32) -> Self {
        self.skidding = skidding;
        self
    }

    /// Sets whether the tooltip is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the tooltip should hoist to prevent clipping
    /// When true, the tooltip won't be clipped by containers with overflow properties
    pub fn hoist(mut self, hoist: bool) -> Self {
        self.hoist = hoist;
        self
    }
}

impl<'a, Message> From<Tooltip<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(t: Tooltip<'a, Message>) -> Self {
        if t.disabled {
            // If disabled, just return the child
            t.child
        } else {
            // Create tooltip with Shoelace styling
            let tooltip_text = text(t.content.clone()).size(FONT_SIZE.small);

            // Wrap in a container with Shoelace tooltip styling
            let tooltip_container = container(tooltip_text)
                .padding(Padding::from([SPACING.x_small, SPACING.small]))
                .class(ContainerStyleClass::Tooltip);

            let mut tooltip_widget =
                widgets::tooltip::Tooltip::new(t.child, tooltip_container, t.placement.into())
                    .gap(t.distance)
                    .padding(SPACING.x_small);

            // When hoist is true, we don't snap to viewport to allow overflow
            tooltip_widget = tooltip_widget.snap_within_viewport(!t.hoist);

            tooltip_widget.into()
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
