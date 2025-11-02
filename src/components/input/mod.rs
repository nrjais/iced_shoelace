use crate::{
    Element,
    theme::{
        Theme,
        input::InputStyleClass,
        pallete::{ColorToken, ColorValue, ColorVariant},
        sizes::{FONT_SIZE, INPUT_HEIGHT, LINE_HEIGHT, SPACING},
        text::TextStyleClass,
    },
};
use iced::Length;
use iced::widget::{Component, component, text, text_input};
use iced_widget::Column;

/// Size variants for input
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl InputSize {
    #[allow(dead_code)]
    fn height(&self) -> f32 {
        match self {
            Self::Small => INPUT_HEIGHT.small,
            Self::Medium => INPUT_HEIGHT.medium,
            Self::Large => INPUT_HEIGHT.large,
        }
    }

    fn font_size(&self) -> f32 {
        match self {
            Self::Small => FONT_SIZE.small,
            Self::Medium => FONT_SIZE.medium,
            Self::Large => FONT_SIZE.large,
        }
    }

    fn spacing(&self) -> f32 {
        match self {
            Self::Small => SPACING.x2_small,
            Self::Medium => SPACING.x_small,
            Self::Large => SPACING.small,
        }
    }
}

/// Input type variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Text,
    Email,
    Number,
    Password,
    Search,
    Tel,
    Url,
}

/// A Shoelace-style input component for iced
///
/// This component implements the Shoelace input with:
/// - Multiple sizes (small, medium, large)
/// - Various input types (text, email, password, number, tel, url, search)
/// - Filled variant (solid background)
/// - Pill variant (fully rounded corners)
/// - Disabled state
/// - Readonly state
/// - Help text support
/// - Label support
/// - Placeholder text
/// - Required field marking
pub struct Input<Message> {
    label: Option<String>,
    value: String,
    placeholder: String,
    input_type: InputType,
    size: InputSize,
    disabled: bool,
    readonly: bool,
    filled: bool,
    pill: bool,
    help_text: Option<String>,
    required: bool,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
}

impl<Message> Input<Message> {
    /// Creates a new input with the given placeholder
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            label: None,
            value: String::new(),
            placeholder: placeholder.into(),
            input_type: InputType::Text,
            size: InputSize::Medium,
            disabled: false,
            readonly: false,
            filled: false,
            pill: false,
            help_text: None,
            required: false,
            on_input: None,
        }
    }

    /// Sets the current value of the input
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the label text shown above the input
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the input type
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }

    /// Sets the input size
    pub fn size(mut self, size: InputSize) -> Self {
        self.size = size;
        self
    }

    /// Sets whether the input is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the input is readonly
    pub fn readonly(mut self, readonly: bool) -> Self {
        self.readonly = readonly;
        self
    }

    /// Sets whether the input uses the filled style (solid background)
    pub fn filled(mut self, filled: bool) -> Self {
        self.filled = filled;
        self
    }

    /// Sets whether the input uses pill style (fully rounded corners)
    pub fn pill(mut self, pill: bool) -> Self {
        self.pill = pill;
        self
    }

    /// Sets the help text shown below the input
    pub fn help_text(mut self, help_text: impl Into<String>) -> Self {
        self.help_text = Some(help_text.into());
        self
    }

    /// Sets the callback that will be invoked when the input value changes
    pub fn on_input<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(String) -> Message,
    {
        self.on_input = Some(Box::new(f));
        self
    }

    /// Sets whether the input is required
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    InputChanged(String),
}

impl<'a, Message> Component<'a, Message, Theme> for Input<Message>
where
    Message: Clone + 'a,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::InputChanged(new_value) => {
                if !self.disabled && !self.readonly {
                    self.value = new_value.clone();
                    self.on_input.as_ref().map(|f| f(new_value))
                } else {
                    None
                }
            }
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'a, Self::Event> {
        let font_size = self.size.font_size();
        let spacing = self.size.spacing();
        let line_height = LINE_HEIGHT.dense;

        let value = self.value.clone();
        let placeholder = self.placeholder.clone();
        let label_text = self.label.clone();
        let help_text = self.help_text.clone();
        let disabled = self.disabled;
        let readonly = self.readonly;
        let filled = self.filled;
        let pill = self.pill;
        let input_type = self.input_type;
        let required = self.required;

        // Create the style class for the input
        let style_class = InputStyleClass {
            size: self.size,
            disabled,
            filled,
            pill,
        };

        // Build the text input control
        let is_password = matches!(input_type, InputType::Password);

        let text_input_control = text_input(&placeholder, &value)
            .size(font_size)
            .line_height(line_height)
            .class(style_class)
            .secure(is_password)
            .padding(if pill {
                [SPACING.x_small, SPACING.medium]
            } else {
                [SPACING.x_small, SPACING.small]
            })
            .on_input_maybe(if !disabled && !readonly {
                Some(Event::InputChanged)
            } else {
                None
            })
            .width(Length::Fill);

        // Build the complete control with optional label and help text
        let mut content = Column::new().spacing(spacing);

        // Add label if present (with required marker)
        if let Some(label) = label_text {
            let label_str = if required {
                format!("{} *", label)
            } else {
                label
            };
            let label_style = text(label_str).size(font_size).class(TextStyleClass {
                color: Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C700)),
            });
            content = content.push(label_style);
        }

        // Add the input control
        content = content.push(text_input_control);

        // Add help text if present
        if let Some(help) = help_text {
            let help_style = text(help).size(FONT_SIZE.small).class(TextStyleClass {
                color: Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C500)),
            });
            content = content.push(help_style);
        }

        content.into()
    }
}

impl<'a, Message> From<Input<Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(input: Input<Message>) -> Self {
        component(input)
    }
}

/// Helper function to create an input
pub fn input<Message>(placeholder: impl Into<String>) -> Input<Message>
where
    Message: Clone,
{
    Input::new(placeholder)
}
