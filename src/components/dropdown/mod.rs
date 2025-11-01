use crate::{
    Element,
    components::{Popup, popup::Placement},
    theme::container::ContainerStyleClass,
};
use std::time::Duration;

/// A Shoelace-style dropdown component for iced
///
/// This component implements the features from Shoelace's dropdown component:
/// - Combines a trigger element with a popup menu
/// - Multiple placement options (inherited from Popup)
/// - Open/close state management
/// - Configurable distance from trigger
/// - Configurable skidding (offset along the trigger)
/// - Flip behavior to stay in viewport
/// - Shift behavior to prevent clipping
/// - Hoist behavior to escape overflow containers
///
/// # Key Differences from Popup
///
/// While Popup is a low-level component for positioning any content,
/// Dropdown is specifically designed for menu interactions:
/// - Automatically styles the menu container
/// - Designed for MenuItem children
/// - Common pattern for navigation and actions
///
/// # Example
///
/// ```rust
/// use iced_shoelace::components::{Dropdown, Menu, MenuItem};
/// use iced_shoelace::components::button::Button;
///
/// // Basic dropdown
/// let my_dropdown = Dropdown::new(
///     Button::new("Options"),
///     Menu::new()
///         .push(MenuItem::new("Option 1").on_select(Message::Option1))
///         .push(MenuItem::new("Option 2").on_select(Message::Option2)),
///     true, // open state
/// );
///
/// // Dropdown with custom placement
/// let custom_dropdown = Dropdown::new(
///     Button::new("Menu"),
///     Menu::new()
///         .push(MenuItem::new("Profile"))
///         .push(MenuItem::new("Settings")),
///     true,
/// )
/// .placement(Placement::BottomEnd)
/// .distance(8.0);
/// ```
pub struct Dropdown<'a, Message> {
    trigger: Element<'a, Message>,
    menu: Element<'a, Message>,
    open: bool,
    placement: Placement,
    distance: f32,
    skidding: f32,
    flip: bool,
    shift: bool,
    hoist: bool,
    style: ContainerStyleClass,
}

impl<'a, Message> Dropdown<'a, Message> {
    /// Creates a new dropdown with the given trigger, menu, and open state
    ///
    /// # Arguments
    ///
    /// * `trigger` - The element that activates the dropdown (typically a Button)
    /// * `menu` - The menu content to display (typically a Menu with MenuItems)
    /// * `open` - Whether the dropdown is currently open
    pub fn new(
        trigger: impl Into<Element<'a, Message>>,
        menu: impl Into<Element<'a, Message>>,
        open: bool,
    ) -> Self {
        Self {
            trigger: trigger.into(),
            menu: menu.into(),
            open,
            placement: Placement::Bottom,
            distance: 4.0,
            skidding: 0.0,
            flip: true,
            shift: true,
            hoist: false,
            style: ContainerStyleClass::Default,
        }
    }

    /// Sets the dropdown placement
    ///
    /// Shoelace supports 12 placements:
    /// - `Top`, `TopStart`, `TopEnd`
    /// - `Bottom`, `BottomStart`, `BottomEnd` (default: Bottom)
    /// - `Left`, `LeftStart`, `LeftEnd`
    /// - `Right`, `RightStart`, `RightEnd`
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the distance between dropdown and trigger (in pixels)
    ///
    /// This is the gap perpendicular to the placement direction.
    /// Default: 4.0 (Shoelace uses 4px for dropdowns)
    pub fn distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }

    /// Sets the skidding (offset along the trigger) in pixels
    ///
    /// This shifts the dropdown along the placement axis.
    /// Positive values shift in the direction of the end alignment.
    /// Default: 0.0
    pub fn skidding(mut self, skidding: f32) -> Self {
        self.skidding = skidding;
        self
    }

    /// Sets whether the dropdown should flip to the opposite side if it doesn't fit
    ///
    /// When true, the dropdown will automatically flip to the opposite placement
    /// if it would overflow the viewport.
    /// Default: true (matches Shoelace default)
    pub fn flip(mut self, flip: bool) -> Self {
        self.flip = flip;
        self
    }

    /// Sets whether the dropdown should shift to stay in viewport
    ///
    /// When true, the dropdown will shift along the placement axis to stay
    /// within the viewport bounds.
    /// Default: true (matches Shoelace default)
    pub fn shift(mut self, shift: bool) -> Self {
        self.shift = shift;
        self
    }

    /// Sets whether the dropdown should be hoisted to the top layer
    ///
    /// When true, the dropdown escapes overflow containers and is positioned
    /// relative to the viewport instead of the parent container.
    /// Default: false
    /// Note: This feature is not yet fully implemented in the current version
    pub fn hoist(mut self, hoist: bool) -> Self {
        self.hoist = hoist;
        self
    }

    /// Sets the container style for the dropdown menu
    ///
    /// Default: ContainerStyleClass::Default
    /// Note: In most cases, you don't need to set this as the Menu
    /// component already has appropriate styling
    pub fn style(mut self, style: ContainerStyleClass) -> Self {
        self.style = style;
        self
    }

    /// Sets whether the dropdown is open (visible)
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl<'a, Message> From<Dropdown<'a, Message>> for Element<'a, Message>
where
    Message: 'a,
{
    fn from(dropdown: Dropdown<'a, Message>) -> Self {
        // Use the Popup component for positioning
        // Popup handles all the positioning logic, viewport constraints, etc.
        let popup = Popup::new(dropdown.trigger, dropdown.menu, dropdown.open)
            .placement(dropdown.placement)
            .distance(dropdown.distance)
            .skidding(dropdown.skidding)
            .flip(dropdown.flip)
            .shift(dropdown.shift)
            .style(dropdown.style)
            .duration(Duration::from_millis(0)); // Show immediately

        popup.into()
    }
}

/// Convenience function to create a dropdown
///
/// # Arguments
///
/// * `trigger` - The element that activates the dropdown (typically a Button)
/// * `menu` - The menu content to display (typically a Menu with MenuItems)
/// * `open` - Whether the dropdown is currently open
///
/// # Example
///
/// ```rust
/// use iced_shoelace::components::{dropdown, menu, menu_item};
/// use iced_shoelace::components::button::Button;
///
/// let my_dropdown = dropdown(
///     Button::new("Menu"),
///     menu()
///         .push(menu_item("New").on_select(Message::New))
///         .push(menu_item("Open").on_select(Message::Open)),
///     true
/// );
/// ```
pub fn dropdown<'a, Message>(
    trigger: impl Into<Element<'a, Message>>,
    menu: impl Into<Element<'a, Message>>,
    open: bool,
) -> Dropdown<'a, Message>
where
    Message: 'a,
{
    Dropdown::new(trigger, menu, open)
}
