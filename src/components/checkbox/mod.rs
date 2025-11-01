use crate::{
    Element,
    theme::{
        Theme,
        checkbox::CheckboxStyleClass,
        pallete::{ColorToken, ColorValue, ColorVariant},
        sizes::{FONT_SIZE, LINE_HEIGHT, SPACING, TOGGLE_SIZE},
        text::TextStyleClass,
    },
};
use iced::widget::{Component, checkbox as iced_checkbox, component, text};
use iced_widget::Column;

/// Size variants for checkboxes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl CheckboxSize {
    fn control_size(&self) -> f32 {
        match self {
            Self::Small => TOGGLE_SIZE.small,
            Self::Medium => TOGGLE_SIZE.medium,
            Self::Large => TOGGLE_SIZE.large,
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

/// A Shoelace-style checkbox component for iced
///
/// This component implements the Shoelace checkbox with:
/// - Multiple sizes (small, medium, large)
/// - Checked, unchecked, and indeterminate states
/// - Disabled state
/// - Help text support
/// - Custom label
pub struct Checkbox<Message> {
    label: String,
    checked: bool,
    indeterminate: bool,
    size: CheckboxSize,
    disabled: bool,
    help_text: Option<String>,
    on_toggle: Option<Box<dyn Fn(bool) -> Message>>,
}

impl<Message> Checkbox<Message> {
    /// Creates a new checkbox with the given label
    pub fn new(label: impl Into<String>, checked: bool) -> Self {
        Self {
            label: label.into(),
            checked,
            indeterminate: false,
            size: CheckboxSize::Medium,
            disabled: false,
            help_text: None,
            on_toggle: None,
        }
    }

    /// Sets the checkbox size
    pub fn size(mut self, size: CheckboxSize) -> Self {
        self.size = size;
        self
    }

    /// Sets whether the checkbox is disabled
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the checkbox is in an indeterminate state
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Sets the help text shown below the checkbox
    pub fn help_text(mut self, help_text: impl Into<String>) -> Self {
        self.help_text = Some(help_text.into());
        self
    }

    /// Sets the callback that will be invoked when the checkbox is toggled
    pub fn on_toggle<F>(mut self, f: F) -> Self
    where
        F: 'static + Fn(bool) -> Message,
    {
        self.on_toggle = Some(Box::new(f));
        self
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    Toggled(bool),
}

impl<'a, Message> Component<'a, Message, Theme> for Checkbox<Message>
where
    Message: Clone + 'a,
{
    type State = ();
    type Event = Event;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Toggled(new_state) => {
                if !self.disabled {
                    self.checked = new_state;
                    // Clear indeterminate when user toggles
                    self.indeterminate = false;
                    self.on_toggle.as_ref().map(|f| f(new_state))
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
        let control_size = self.size.control_size();

        let label_text = self.label.clone();
        let checked = self.checked;
        let indeterminate = self.indeterminate;
        let disabled = self.disabled;
        let help_text = self.help_text.clone();

        // Create the style class for the checkbox
        let style_class = CheckboxStyleClass {
            is_checked: checked,
            is_indeterminate: indeterminate,
            is_disabled: disabled,
        };

        // Build the main checkbox control with label
        let checkbox_control = iced_checkbox(label_text, checked)
            .size(control_size)
            .text_size(font_size)
            .text_line_height(line_height)
            .class(style_class)
            .on_toggle_maybe(if !disabled {
                Some(Event::Toggled)
            } else {
                None
            });

        // Build the complete control with optional help text
        let mut content = Column::new().spacing(spacing);
        content = content.push(checkbox_control);

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

impl<'a, Message> From<Checkbox<Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(checkbox: Checkbox<Message>) -> Self {
        component(checkbox)
    }
}

/// Helper function to create a checkbox
pub fn checkbox<Message>(label: impl Into<String>, checked: bool) -> Checkbox<Message>
where
    Message: Clone,
{
    Checkbox::new(label, checked)
}
