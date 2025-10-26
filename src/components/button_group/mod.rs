use crate::Element;
use crate::components::button::Button;
use iced::widget::{Row, text};
use iced_widget::Column;

use crate::theme::Theme;

/// Position of a button within a button group
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonPosition {
    /// First button in the group (rounded left corners)
    First,
    /// Middle button in the group (no rounded corners)
    Middle,
    /// Last button in the group (rounded right corners)
    Last,
    /// Only button in the group (all corners rounded)
    Only,
}

/// A button group component that visually groups buttons together
///
/// This component implements Shoelace's button-group features:
/// - Groups buttons with connected borders
/// - Optional label above the group
/// - Buttons appear as a single cohesive unit
/// - First button has left-rounded corners, last has right-rounded corners
/// - Middle buttons have square corners for a connected appearance
pub struct ButtonGroup<'a, Message> {
    buttons: Vec<Button<Message>>,
    label: Option<String>,
    spacing: f32,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, Message> ButtonGroup<'a, Message> {
    /// Creates a new button group
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
            label: None,
            spacing: -1.0, // Negative spacing to overlap borders
            _phantom: std::marker::PhantomData,
        }
    }

    /// Creates a button group with buttons
    pub fn with_buttons(buttons: Vec<Button<Message>>) -> Self {
        Self {
            buttons,
            label: None,
            spacing: -1.0, // Negative spacing to overlap borders
            _phantom: std::marker::PhantomData,
        }
    }

    /// Adds a button to the group
    pub fn push(mut self, button: Button<Message>) -> Self {
        self.buttons.push(button);
        self
    }

    /// Sets the label text displayed above the button group
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the spacing between buttons (default is -1 for overlapped borders)
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Builds the button group into an Element
    pub fn build(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let theme = Theme::default();
        let tokens = theme.tokens();
        let button_count = self.buttons.len();
        let border_radius = tokens.border_radius.medium;

        // Create the button row with connected appearance
        let mut button_row = Row::new().spacing(self.spacing);

        // Add all buttons with position-based border radius
        for (index, button) in self.buttons.into_iter().enumerate() {
            let position = if button_count == 1 {
                ButtonPosition::Only
            } else if index == 0 {
                ButtonPosition::First
            } else if index == button_count - 1 {
                ButtonPosition::Last
            } else {
                ButtonPosition::Middle
            };

            // Apply border radius based on position
            let radius = match position {
                ButtonPosition::First => iced::border::Radius {
                    top_left: border_radius,
                    top_right: 0.0,
                    bottom_right: 0.0,
                    bottom_left: border_radius,
                },
                ButtonPosition::Middle => iced::border::Radius::default(),
                ButtonPosition::Last => iced::border::Radius {
                    top_left: 0.0,
                    top_right: border_radius,
                    bottom_right: border_radius,
                    bottom_left: 0.0,
                },
                ButtonPosition::Only => iced::border::Radius::from(border_radius),
            };

            let styled_button = button.border_radius(radius);
            button_row = button_row.push(styled_button);
        }

        // Wrap in a container-like row
        let group_container: Element<'a, Message> = button_row.into();

        // If there's a label, add it above the button group
        if let Some(label_text) = self.label {
            let label: Element<'a, Message> = text(label_text).size(tokens.font_size.small).into();

            Column::new()
                .spacing(tokens.spacing.x2_small)
                .push(label)
                .push(group_container)
                .into()
        } else {
            group_container
        }
    }
}

impl<'a, Message> Default for ButtonGroup<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message> From<ButtonGroup<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(button_group: ButtonGroup<'a, Message>) -> Self {
        button_group.build()
    }
}

/// Convenience function to create a button group
pub fn button_group<'a, Message>() -> ButtonGroup<'a, Message>
where
    Message: 'a,
{
    ButtonGroup::new()
}

/// Convenience function to create a button group with buttons
pub fn button_group_with<'a, Message>(buttons: Vec<Button<Message>>) -> ButtonGroup<'a, Message>
where
    Message: 'a,
{
    ButtonGroup::with_buttons(buttons)
}
