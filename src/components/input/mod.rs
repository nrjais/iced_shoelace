use crate::components::button::Button;
use crate::{
    Element,
    theme::{
        Theme,
        button::ButtonVariant,
        input::InputStyleClass,
        pallete::{ColorToken, ColorValue, ColorVariant},
        sizes::{FONT_SIZE, INPUT_HEIGHT, LINE_HEIGHT, SPACING},
        text::TextStyleClass,
    },
};
use iced::widget::{Component, component, row, text, text_input};
use iced::{Alignment, Length};
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
/// - Clearable button
/// - Password toggle button
/// - Min/max length constraints
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
    clearable: bool,
    password_toggle: bool,
    password_visible: bool,
    min_length: Option<usize>,
    max_length: Option<usize>,
    required: bool,
    on_input: Option<Box<dyn Fn(String) -> Message>>,
    on_clear: Option<Message>,
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
            clearable: false,
            password_toggle: false,
            password_visible: false,
            min_length: None,
            max_length: None,
            required: false,
            on_input: None,
            on_clear: None,
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

    /// Sets whether the input should have a clear button
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Sets the callback that will be invoked when the clear button is pressed
    pub fn on_clear(mut self, message: Message) -> Self {
        self.on_clear = Some(message);
        self
    }

    /// Sets whether password inputs should have a toggle button to show/hide
    pub fn password_toggle(mut self, password_toggle: bool) -> Self {
        self.password_toggle = password_toggle;
        self
    }

    /// Sets the minimum length for the input value
    pub fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }

    /// Sets the maximum length for the input value
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
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
    ClearPressed,
    PasswordTogglePressed,
}

impl<'a, Message> Component<'a, Message, Theme> for Input<Message>
where
    Message: Clone + 'a,
{
    type State = bool; // Track password visibility state
    type Event = Event;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::InputChanged(new_value) => {
                if !self.disabled && !self.readonly {
                    // Apply max_length constraint
                    let constrained_value = if let Some(max_len) = self.max_length {
                        if new_value.len() > max_len {
                            return None; // Don't update if exceeds max length
                        }
                        new_value
                    } else {
                        new_value
                    };

                    self.value = constrained_value.clone();
                    self.on_input.as_ref().map(|f| f(constrained_value))
                } else {
                    None
                }
            }
            Event::ClearPressed => {
                if !self.disabled && !self.readonly {
                    self.value.clear();
                    self.on_clear.clone()
                } else {
                    None
                }
            }
            Event::PasswordTogglePressed => {
                *state = !*state; // Toggle password visibility
                self.password_visible = *state;
                None
            }
        }
    }

    fn view(&self, state: &Self::State) -> Element<'a, Self::Event> {
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
        let clearable = self.clearable;
        let password_toggle = self.password_toggle;
        let has_value = !value.is_empty();
        let required = self.required;
        let min_length = self.min_length;
        let max_length = self.max_length;

        // Create the style class for the input
        let style_class = InputStyleClass {
            size: self.size,
            disabled,
            filled,
            pill,
        };

        // Build the text input control
        let is_password = matches!(input_type, InputType::Password);
        let show_password = is_password && password_toggle && *state;

        let text_input_control = text_input(&placeholder, &value)
            .size(font_size)
            .line_height(line_height)
            .class(style_class)
            .secure(is_password && !show_password)
            .on_input_maybe(if !disabled && !readonly {
                Some(Event::InputChanged)
            } else {
                None
            })
            .width(Length::Fill);

        // Build input row with input and buttons
        let mut input_row = row![text_input_control]
            .spacing(SPACING.x2_small)
            .align_y(Alignment::Center);

        // Add clear button if clearable and has value
        if clearable && has_value && !disabled && !readonly {
            let clear_btn = Button::new("✕")
                .variant(ButtonVariant::Text)
                .on_press(Event::ClearPressed);
            input_row = input_row.push(clear_btn);
        }

        // Add password toggle button if password input with toggle enabled
        if is_password && password_toggle && !disabled {
            let toggle_text = if *state { "👁" } else { "👁‍🗨" };
            let toggle_btn = Button::new(toggle_text)
                .variant(ButtonVariant::Text)
                .on_press(Event::PasswordTogglePressed);
            input_row = input_row.push(toggle_btn);
        }

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

        // Add the input row
        content = content.push(input_row);

        // Add help text if present (with validation info)
        let help_with_validation = if let Some(help) = help_text {
            if let (Some(min), Some(max)) = (min_length, max_length) {
                Some(format!("{} ({}-{} characters)", help, min, max))
            } else if let Some(min) = min_length {
                Some(format!("{} (min {} characters)", help, min))
            } else if let Some(max) = max_length {
                Some(format!("{} (max {} characters)", help, max))
            } else {
                Some(help)
            }
        } else if let (Some(min), Some(max)) = (min_length, max_length) {
            Some(format!("{}-{} characters", min, max))
        } else if let Some(min) = min_length {
            Some(format!("Minimum {} characters", min))
        } else if let Some(max) = max_length {
            Some(format!("Maximum {} characters", max))
        } else {
            None
        };

        if let Some(help) = help_with_validation {
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
