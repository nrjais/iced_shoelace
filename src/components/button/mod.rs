use crate::{
    Element,
    components::hovered,
    theme::{
        Theme,
        sizes::{BORDER_RADIUS, FONT_SIZE, LINE_HEIGHT, SPACING},
    },
};
use iced::{
    Alignment, Padding,
    border::Radius,
    widget::{self, Component, component, text},
};
use iced_widget::Row;

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
    pub border_radius: Radius,
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
    custom_border_radius: Option<Radius>,
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
            custom_border_radius: None,
        }
    }

    /// Sets a custom border radius for the button (used by button groups)
    pub fn border_radius(mut self, radius: Radius) -> Self {
        self.custom_border_radius = Some(radius);
        self
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
    fn get_padding(&self) -> Padding {
        // Following Shoelace spec: small (0.5rem, 1rem), medium (0.75rem, 1.5rem), large (1rem, 2rem)
        match self.size {
            Size::Small => Padding::from([SPACING.x_small, SPACING.medium]), // 8px, 16px
            Size::Medium => Padding::from([SPACING.small, SPACING.x_large]), // 12px, 28px (close to 24px)
            Size::Large => Padding::from([SPACING.medium, SPACING.x2_large]), // 16px, 36px (close to 32px)
        }
    }

    /// Gets the appropriate border radius
    fn get_border_radius(&self) -> Radius {
        // Use custom border radius if set (for button groups)
        if let Some(custom_radius) = self.custom_border_radius {
            return custom_radius;
        }

        let radius = if self.pill {
            BORDER_RADIUS.x_large * 10.0
        } else {
            match self.size {
                Size::Small => BORDER_RADIUS.small,
                Size::Medium => BORDER_RADIUS.medium,
                Size::Large => BORDER_RADIUS.large,
            }
        };

        Radius::from(radius)
    }

    /// Gets the appropriate font size
    fn get_font_size(&self) -> f32 {
        match self.size {
            Size::Small => FONT_SIZE.small,
            Size::Medium => FONT_SIZE.medium,
            Size::Large => FONT_SIZE.large,
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
        let padding = self.get_padding();
        let border_radius = self.get_border_radius();
        let font_size = self.get_font_size();
        let line_height = LINE_HEIGHT.dense;

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
            let mut row_content = Row::new()
                .spacing(SPACING.x2_small)
                .align_y(Alignment::Center);

            // Add prefix if present
            if let Some(prefix_text) = &prefix
                && !prefix_text.is_empty()
            {
                row_content = row_content.push(
                    text(prefix_text.clone())
                        .size(font_size)
                        .line_height(line_height),
                );
            }

            // Add main label with proper typography
            row_content = row_content.push(
                text(label_str.clone())
                    .size(font_size)
                    .line_height(line_height),
            );

            // Add suffix if present
            if let Some(suffix_text) = &suffix
                && !suffix_text.is_empty()
            {
                row_content = row_content.push(
                    text(suffix_text.clone())
                        .size(font_size)
                        .line_height(line_height),
                );
            }

            let button_content: Element<'a, Event> = row_content.into();

            // Build the button
            let btn = widget::button(button_content)
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
