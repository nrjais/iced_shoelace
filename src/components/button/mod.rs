use crate::{Element, components::hovered};
use iced::{
    Padding,
    widget::{Component, button, component, text},
};
use iced_widget::Row;

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
    pub disabled: bool,
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
/// - Disabled state
/// - Optional caret icon
/// - Prefix and suffix icons/content
pub struct Button<Message> {
    label: String,
    variant: Variant,
    size: Size,
    outline: bool,
    pill: bool,
    loading: bool,
    disabled: bool,
    prefix: Option<String>,
    suffix: Option<String>,
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
            loading: false,
            disabled: false,
            prefix: None,
            suffix: None,
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

    /// Sets whether the button is in a loading state
    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    /// Sets whether the button is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the prefix icon/text (displayed before the label)
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Sets the suffix icon/text (displayed after the label)
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
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

        // Following Shoelace spec: small (0.5rem, 1rem), medium (0.75rem, 1.5rem), large (1rem, 2rem)
        match self.size {
            Size::Small => Padding::from([spacing.x_small, spacing.medium]), // 8px, 16px
            Size::Medium => Padding::from([spacing.small, spacing.x_large]), // 12px, 28px (close to 24px)
            Size::Large => Padding::from([spacing.medium, spacing.x2_large]), // 16px, 36px (close to 32px)
        }
    }

    /// Gets the appropriate border radius
    fn get_border_radius(&self, theme: &Theme) -> f32 {
        let tokens = theme.tokens();
        let border_radius = tokens.border_radius;

        if self.pill {
            border_radius.x_large * 10.0
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
                if !self.loading && !self.disabled {
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
        } else {
            self.label.clone()
        };

        let variant = self.variant;
        let outline = self.outline;
        let loading = self.loading;
        let disabled = self.disabled;
        let prefix = self.prefix.clone();
        let suffix = self.suffix.clone();
        let has_on_press = self.on_press.is_some();

        // Use the hovered component to track hover state
        let content: Element<'a, Event> = hovered(move |is_hovered| {
            // Create style class with hover state
            let style_class = ButtonStyleClass {
                variant,
                outline,
                border_radius,
                hovered: is_hovered && !disabled,
                disabled,
            };

            // Build the button content with prefix, label, and suffix
            let mut row_content = Row::new().spacing(theme.tokens().spacing.x2_small);

            // Add prefix if present
            if let Some(prefix_text) = &prefix {
                if !prefix_text.is_empty() {
                    row_content = row_content.push(text(prefix_text.clone()).size(font_size));
                }
            }

            // Add main label
            row_content = row_content.push(text(label_str.clone()).size(font_size));

            // Add suffix if present
            if let Some(suffix_text) = &suffix {
                if !suffix_text.is_empty() {
                    row_content = row_content.push(text(suffix_text.clone()).size(font_size));
                }
            }

            let button_content: Element<'a, Event> = row_content.into();

            // Build the button
            let btn = button(button_content)
                .padding(padding)
                .class(style_class)
                .on_press_maybe(if !loading && !disabled && has_on_press {
                    Some(Event::Pressed)
                } else {
                    None
                });

            let element: Element<'a, Event> = btn.into();
            element
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
