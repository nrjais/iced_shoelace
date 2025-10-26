pub mod catalog;

use crate::{Element, components::hovered};
use iced::{
    Length, Padding,
    widget::{Component, button, component, text},
};

use crate::theme::Theme;

/// Button variant types matching Shoelace design system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    #[default]
    Default,
    Primary,
    Success,
    Neutral,
    Warning,
    Danger,
    Text,
}

/// Button size options matching Shoelace design system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    Small,
    #[default]
    Medium,
    Large,
}

/// Style class for custom button styling
#[derive(Debug, Clone, Copy)]
pub struct ButtonStyleClass {
    pub variant: Variant,
    pub outline: bool,
    pub border_radius: f32,
    pub hovered: bool,
    pub pressed: bool,
}

/// A Shoelace-style button component for iced
///
/// This component implements all the features from Shoelace's button component:
/// - Multiple variants (default, primary, success, neutral, warning, danger, text)
/// - Multiple sizes (small, medium, large)
/// - Outline style
/// - Pill (fully rounded) style
/// - Circle style
/// - Loading state
/// - Optional caret icon
pub struct Button<Message> {
    label: String,
    variant: Variant,
    size: Size,
    outline: bool,
    pill: bool,
    circle: bool,
    loading: bool,
    caret: bool,
    width: Length,
    on_press: Option<Message>,
}

impl<Message> Button<Message> {
    /// Creates a new button with the given label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            variant: Variant::Default,
            size: Size::Medium,
            outline: false,
            pill: false,
            circle: false,
            loading: false,
            caret: false,
            width: Length::Shrink,
            on_press: None,
        }
    }

    /// Sets the button variant
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the button size
    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    /// Sets whether the button has an outline style
    pub fn outline(mut self, outline: bool) -> Self {
        self.outline = outline;
        self
    }

    /// Sets whether the button has fully rounded corners (pill style)
    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }

    /// Sets whether the button is circular
    pub fn circle(mut self, circle: bool) -> Self {
        self.circle = circle;
        self
    }

    /// Sets whether the button is in a loading state
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets whether the button displays a caret icon
    pub fn caret(mut self, caret: bool) -> Self {
        self.caret = caret;
        self
    }

    /// Sets the button width
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the message that will be produced when the button is pressed
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Gets the appropriate padding based on size
    fn get_padding(&self, theme: &Theme) -> Padding {
        let tokens = theme.tokens();
        let spacing = tokens.spacing;

        if self.circle {
            // Circle buttons use equal padding
            let pad = match self.size {
                Size::Small => spacing.x_small,
                Size::Medium => spacing.small,
                Size::Large => spacing.medium,
            };
            Padding::from(pad)
        } else {
            // Regular buttons use horizontal and vertical padding
            match self.size {
                Size::Small => Padding::from([spacing.x3_small, spacing.small]),
                Size::Medium => Padding::from([spacing.x2_small, spacing.medium]),
                Size::Large => Padding::from([spacing.x_small, spacing.large]),
            }
        }
    }

    /// Gets the appropriate border radius
    fn get_border_radius(&self, theme: &Theme) -> f32 {
        let tokens = theme.tokens();
        let border_radius = tokens.border_radius;

        if self.pill || self.circle {
            999.0 // Very large radius for fully rounded corners
        } else {
            match self.size {
                Size::Small => border_radius.small,
                Size::Medium => border_radius.medium,
                Size::Large => border_radius.large,
            }
        }
    }

    /// Gets the appropriate font size
    fn get_font_size(&self, theme: &Theme) -> f32 {
        let tokens = theme.tokens();
        let font_size = tokens.font_size;

        match self.size {
            Size::Small => font_size.small,
            Size::Medium => font_size.medium,
            Size::Large => font_size.large,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Pressed,
}

impl<'a, Message> Component<'a, Message, Theme> for Button<Message>
where
    Message: Clone,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Pressed => {
                if !self.loading {
                    self.on_press.clone()
                } else {
                    None
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'a, Self::Event> {
        let theme = Theme::default();
        let padding = self.get_padding(&theme);
        let border_radius = self.get_border_radius(&theme);
        let font_size = self.get_font_size(&theme);

        // Build label text with optional caret
        let label_str = if self.loading {
            "⋯".to_string()
        } else if self.circle {
            self.label.clone()
        } else if self.caret {
            format!("{} ▼", self.label)
        } else {
            self.label.clone()
        };

        let variant = self.variant;
        let outline = self.outline;
        let loading = self.loading;
        let has_on_press = self.on_press.is_some();
        let width = self.width;

        // Use the hovered component to track hover state
        let content: Element<'a, Event> = hovered(move |is_hovered| {
            // Create style class with hover state
            let style_class = ButtonStyleClass {
                variant,
                outline,
                border_radius,
                hovered: is_hovered,
                pressed: false,
            };

            // Build the button
            button(text(label_str.clone()).size(font_size))
                .padding(padding)
                .width(width)
                .class(style_class)
                .on_press_maybe(if !loading && has_on_press {
                    Some(Event::Pressed)
                } else {
                    None
                })
        });

        content.map(|_message| Event::Pressed)
    }
}

impl<'a, Message> From<Button<Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(button: Button<Message>) -> Self {
        component(button)
    }
}

// Convenience constructors for common button types
impl<Message> Button<Message> {
    /// Creates a new primary button
    pub fn primary(label: impl Into<String>) -> Self {
        Self::new(label).variant(Variant::Primary)
    }

    /// Creates a new success button
    pub fn success(label: impl Into<String>) -> Self {
        Self::new(label).variant(Variant::Success)
    }

    /// Creates a new danger button
    pub fn danger(label: impl Into<String>) -> Self {
        Self::new(label).variant(Variant::Danger)
    }

    /// Creates a new warning button
    pub fn warning(label: impl Into<String>) -> Self {
        Self::new(label).variant(Variant::Warning)
    }

    /// Creates a new neutral button
    pub fn neutral(label: impl Into<String>) -> Self {
        Self::new(label).variant(Variant::Neutral)
    }

    /// Creates a new text button
    pub fn text_button(label: impl Into<String>) -> Self {
        Self::new(label).variant(Variant::Text)
    }
}
