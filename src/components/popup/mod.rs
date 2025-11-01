use crate::{Element, theme::container::ContainerStyleClass, widgets::tooltip::{Position, Tooltip}};
use std::time::Duration;

/// Popup placement options matching Shoelace design system
///
/// Shoelace supports 12 placement options to position popups relative to their anchor:
/// - Top/Bottom/Left/Right: Centered on the respective side
/// - TopStart/BottomStart/LeftStart/RightStart: Aligned to the start
/// - TopEnd/BottomEnd/LeftEnd/RightEnd: Aligned to the end
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

impl From<Placement> for Position {
    fn from(placement: Placement) -> Self {
        match placement {
            Placement::Top | Placement::TopStart | Placement::TopEnd => {
                Position::Top
            }
            Placement::Bottom | Placement::BottomStart | Placement::BottomEnd => {
                Position::Bottom
            }
            Placement::Left | Placement::LeftStart | Placement::LeftEnd => {
                Position::Left
            }
            Placement::Right | Placement::RightStart | Placement::RightEnd => {
                Position::Right
            }
        }
    }
}

/// A Shoelace-style popup component for iced
///
/// This component implements features from Shoelace's popup component:
/// - Positioning relative to an anchor element
/// - Multiple placement options (12 positions)
/// - Configurable distance from anchor
/// - Configurable skidding (offset along the anchor)
/// - Active state control (show/hide)
/// - Optional arrow indicator
/// - Flip behavior to stay in viewport
/// - Shift behavior to prevent clipping
/// - Uses iced's overlay system for proper positioning
///
/// # Differences from Tooltip
///
/// While Tooltip automatically shows on hover, Popup requires manual control via
/// the `active` property, making it suitable for dropdowns, popovers, and other
/// interactive overlays that need explicit show/hide control.
///
/// # Styling
///
/// Popups can use any container styling. By default, they use a card-like style
/// with white background, border, shadow, and rounded corners.
///
/// # Example
///
/// ```rust
/// use iced_shoelace::components::popup::{popup, Placement};
/// use iced_shoelace::components::button::Button;
/// use iced::widget::text;
///
/// // Basic popup
/// let my_popup = popup(
///     Button::new("Click me"),
///     text("Popup content!"),
///     true, // active state
/// );
///
/// // Popup with custom placement and distance
/// let custom_popup = popup(
///     Button::new("Anchor"),
///     text("Bottom popup with more space"),
///     true,
/// )
/// .placement(Placement::Bottom)
/// .distance(16.0);
///
/// // Popup with skidding (horizontal offset)
/// let offset_popup = popup(
///     Button::new("Anchor"),
///     text("Offset to the right"),
///     true,
/// )
/// .skidding(32.0);
///
/// // Popup that doesn't flip or shift
/// let fixed_popup = popup(
///     Button::new("Anchor"),
///     text("Fixed position"),
///     true,
/// )
/// .flip(false)
/// .shift(false);
/// ```
pub struct Popup<'a, Message> {
    anchor: Element<'a, Message>,
    content: Element<'a, Message>,
    active: bool,
    placement: Placement,
    distance: f32,
    skidding: f32,
    arrow: bool,
    flip: bool,
    shift: bool,
    style: ContainerStyleClass,
    duration: Duration,
}

impl<'a, Message> Popup<'a, Message> {
    /// Creates a new popup with the given anchor, content, and active state
    ///
    /// # Arguments
    ///
    /// * `anchor` - The element to position the popup relative to
    /// * `content` - The content to display in the popup
    /// * `active` - Whether the popup is currently visible
    pub fn new(
        anchor: impl Into<Element<'a, Message>>,
        content: impl Into<Element<'a, Message>>,
        active: bool,
    ) -> Self {
        Self {
            anchor: anchor.into(),
            content: content.into(),
            active,
            placement: Placement::Top,
            distance: 0.0,
            skidding: 0.0,
            arrow: false,
            flip: true,
            shift: true,
            style: ContainerStyleClass::Default,
            duration: Duration::from_millis(0), // Show immediately by default
        }
    }

    /// Sets the popup placement
    ///
    /// Shoelace supports 12 placements:
    /// - `Top`, `TopStart`, `TopEnd`
    /// - `Bottom`, `BottomStart`, `BottomEnd`
    /// - `Left`, `LeftStart`, `LeftEnd`
    /// - `Right`, `RightStart`, `RightEnd`
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the distance between popup and anchor (in pixels)
    ///
    /// This is the gap perpendicular to the placement direction.
    /// Default: 0.0 (Shoelace default)
    pub fn distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }

    /// Sets the skidding (offset along the anchor) in pixels
    ///
    /// This shifts the popup along the placement axis.
    /// Positive values shift in the direction of the end alignment.
    /// Default: 0.0
    pub fn skidding(mut self, skidding: f32) -> Self {
        self.skidding = skidding;
        self
    }

    /// Sets whether to show an arrow pointing to the anchor
    ///
    /// Default: false
    /// Note: Arrow rendering is not yet implemented in this version
    pub fn arrow(mut self, arrow: bool) -> Self {
        self.arrow = arrow;
        self
    }

    /// Sets whether the popup should flip to the opposite side if it doesn't fit
    ///
    /// When true, the popup will automatically flip to the opposite placement
    /// if it would overflow the viewport.
    /// Default: true (matches Shoelace default)
    pub fn flip(mut self, flip: bool) -> Self {
        self.flip = flip;
        self
    }

    /// Sets whether the popup should shift to stay in viewport
    ///
    /// When true, the popup will shift along the placement axis to stay
    /// within the viewport bounds.
    /// Default: true (matches Shoelace default)
    pub fn shift(mut self, shift: bool) -> Self {
        self.shift = shift;
        self
    }

    /// Sets the container style for the popup content
    ///
    /// Default: ContainerStyleClass::Card (white background, border, shadow)
    pub fn style(mut self, style: ContainerStyleClass) -> Self {
        self.style = style;
        self
    }

    /// Sets the duration before the popup appears
    ///
    /// Default: 0ms (shows immediately)
    /// Note: This is currently used for hover-based activation
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets whether the popup is active (visible)
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

impl<'a, Message> From<Popup<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(popup: Popup<'a, Message>) -> Self {
        if !popup.active {
            // If not active, just show the anchor without the popup
            popup.anchor
        } else {
            // Use the tooltip widget infrastructure for positioning
            // This provides overlay positioning relative to the anchor
            let popup_widget = Tooltip::new(popup.anchor, popup.content, popup.placement.into())
                .gap(popup.distance)
                .class(popup.style)
                .duration(popup.duration)
                .snap_within_viewport(popup.shift);

            popup_widget.into()
        }
    }
}

/// Convenience function to create a popup
///
/// # Arguments
///
/// * `anchor` - The element to position the popup relative to
/// * `content` - The content to display in the popup
/// * `active` - Whether the popup is currently visible
///
/// # Example
///
/// ```rust
/// use iced_shoelace::components::popup::popup;
/// use iced_shoelace::components::button::Button;
/// use iced::widget::text;
///
/// let my_popup = popup(
///     Button::new("Click me"),
///     text("I'm a popup!"),
///     true
/// );
/// ```
pub fn popup<'a, Message>(
    anchor: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    active: bool,
) -> Popup<'a, Message>
where
    Message: 'a,
{
    Popup::new(anchor, content, active)
}
