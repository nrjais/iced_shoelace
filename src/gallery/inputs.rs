use iced::alignment;
use iced_widget::{Row, column, text};

use crate::components::input::{Input, InputSize, InputType};
use crate::{Element, Message};

#[derive(Debug, Clone)]
pub struct InputState {
    pub basic: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub small: String,
    pub medium: String,
    pub large: String,
    pub text: String,
    pub email_type: String,
    pub password_type: String,
    pub number: String,
    pub tel: String,
    pub url: String,
    pub filled: String,
    pub pill_standard: String,
    pub pill: String,
    pub first_name: String,
    pub last_name: String,
    pub search: String,
    pub combo: String,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            basic: String::new(),
            name: String::new(),
            email: "user@example.com".to_string(),
            password: String::new(),
            small: String::new(),
            medium: String::new(),
            large: String::new(),
            text: String::new(),
            email_type: String::new(),
            password_type: String::new(),
            number: String::new(),
            tel: String::new(),
            url: String::new(),
            filled: String::new(),
            pill_standard: String::new(),
            pill: String::new(),
            first_name: String::new(),
            last_name: String::new(),
            search: String::new(),
            combo: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputMessage {
    BasicChanged(String),
    NameChanged(String),
    EmailChanged(String),
    PasswordChanged(String),
    SmallChanged(String),
    MediumChanged(String),
    LargeChanged(String),
    TextChanged(String),
    EmailTypeChanged(String),
    PasswordTypeChanged(String),
    NumberChanged(String),
    TelChanged(String),
    UrlChanged(String),
    FilledChanged(String),
    PillStandardChanged(String),
    PillChanged(String),
    FirstNameChanged(String),
    LastNameChanged(String),
    SearchChanged(String),
    ComboChanged(String),
}

pub fn handle_input_message(state: &mut InputState, message: InputMessage) {
    match message {
        InputMessage::BasicChanged(value) => state.basic = value,
        InputMessage::NameChanged(value) => state.name = value,
        InputMessage::EmailChanged(value) => state.email = value,
        InputMessage::PasswordChanged(value) => state.password = value,
        InputMessage::SmallChanged(value) => state.small = value,
        InputMessage::MediumChanged(value) => state.medium = value,
        InputMessage::LargeChanged(value) => state.large = value,
        InputMessage::TextChanged(value) => state.text = value,
        InputMessage::EmailTypeChanged(value) => state.email_type = value,
        InputMessage::PasswordTypeChanged(value) => state.password_type = value,
        InputMessage::NumberChanged(value) => state.number = value,
        InputMessage::TelChanged(value) => state.tel = value,
        InputMessage::UrlChanged(value) => state.url = value,
        InputMessage::FilledChanged(value) => state.filled = value,
        InputMessage::PillStandardChanged(value) => state.pill_standard = value,
        InputMessage::PillChanged(value) => state.pill = value,
        InputMessage::FirstNameChanged(value) => state.first_name = value,
        InputMessage::LastNameChanged(value) => state.last_name = value,
        InputMessage::SearchChanged(value) => state.search = value,
        InputMessage::ComboChanged(value) => state.combo = value,
    }
}

pub fn view(state: &InputState) -> Element<'_, Message> {
    let title = text("Inputs").size(32);
    let description = text("Input fields allow users to enter text data").size(14);

    // Basic input
    let basic_title = text("Basic").size(24);
    let basic_input = Input::new("Type something...")
        .value(&state.basic)
        .on_input(|value| Message::Input(InputMessage::BasicChanged(value)));

    // With Label
    let label_title = text("With Label").size(24);
    let label_input = Input::new("Enter your name")
        .label("Name")
        .value(&state.name)
        .on_input(|value| Message::Input(InputMessage::NameChanged(value)));

    // With Value
    let value_title = text("With Value").size(24);
    let value_input = Input::new("Enter your email")
        .label("Email")
        .value(&state.email)
        .on_input(|value| Message::Input(InputMessage::EmailChanged(value)));

    // Help Text
    let help_text_title = text("Help Text").size(24);
    let help_text_input = Input::new("Enter your password")
        .label("Password")
        .input_type(InputType::Password)
        .value(&state.password)
        .help_text("Password must be at least 8 characters")
        .on_input(|value| Message::Input(InputMessage::PasswordChanged(value)));

    // Sizes
    let sizes_title = text("Sizes").size(24);
    let sizes_column = column![
        Input::new("Small input")
            .size(InputSize::Small)
            .value(&state.small)
            .on_input(|value| Message::Input(InputMessage::SmallChanged(value))),
        Input::new("Medium input (default)")
            .size(InputSize::Medium)
            .value(&state.medium)
            .on_input(|value| Message::Input(InputMessage::MediumChanged(value))),
        Input::new("Large input")
            .size(InputSize::Large)
            .value(&state.large)
            .on_input(|value| Message::Input(InputMessage::LargeChanged(value))),
    ]
    .spacing(15);

    // Input Types
    let types_title = text("Input Types").size(24);
    let types_column = column![
        Input::new("Enter text")
            .label("Text")
            .input_type(InputType::Text)
            .value(&state.text)
            .on_input(|value| Message::Input(InputMessage::TextChanged(value))),
        Input::new("Enter email")
            .label("Email")
            .input_type(InputType::Email)
            .value(&state.email_type)
            .on_input(|value| Message::Input(InputMessage::EmailTypeChanged(value))),
        Input::new("Enter password")
            .label("Password")
            .input_type(InputType::Password)
            .value(&state.password_type)
            .on_input(|value| Message::Input(InputMessage::PasswordTypeChanged(value))),
        Input::new("Enter number")
            .label("Number")
            .input_type(InputType::Number)
            .value(&state.number)
            .on_input(|value| Message::Input(InputMessage::NumberChanged(value))),
        Input::new("Enter phone")
            .label("Telephone")
            .input_type(InputType::Tel)
            .value(&state.tel)
            .on_input(|value| Message::Input(InputMessage::TelChanged(value))),
        Input::new("Enter URL")
            .label("URL")
            .input_type(InputType::Url)
            .value(&state.url)
            .on_input(|value| Message::Input(InputMessage::UrlChanged(value))),
    ]
    .spacing(15);

    // Filled Variant
    let filled_title = text("Filled").size(24);
    let filled_input = Input::new("Type something...")
        .label("Filled Input")
        .filled(true)
        .value(&state.filled)
        .on_input(|value| Message::Input(InputMessage::FilledChanged(value)));

    // Pill Variant
    let pill_title = text("Pill").size(24);
    let pill_row = Row::with_children([
        Input::new("Standard")
            .size(InputSize::Small)
            .value(&state.pill_standard)
            .on_input(|value| Message::Input(InputMessage::PillStandardChanged(value)))
            .into(),
        Input::new("Pill shaped")
            .size(InputSize::Small)
            .pill(true)
            .value(&state.pill)
            .on_input(|value| Message::Input(InputMessage::PillChanged(value)))
            .into(),
    ])
    .spacing(20)
    .align_y(alignment::Vertical::Center);

    // Disabled
    let disabled_title = text("Disabled").size(24);
    let disabled_input = Input::new("You can't type here")
        .label("Disabled Input")
        .value("Disabled value")
        .disabled(true);

    // Readonly
    let readonly_title = text("Readonly").size(24);
    let readonly_input = Input::new("You can't edit this")
        .label("Readonly Input")
        .value("This is readonly")
        .readonly(true);

    // Different placeholders
    let placeholders_title = text("Different Placeholders").size(24);
    let placeholders_column = column![
        Input::new("Enter your first name")
            .label("First Name")
            .value(&state.first_name)
            .on_input(|value| Message::Input(InputMessage::FirstNameChanged(value))),
        Input::new("Enter your last name")
            .label("Last Name")
            .value(&state.last_name)
            .on_input(|value| Message::Input(InputMessage::LastNameChanged(value))),
        Input::new("Search for products...")
            .label("Search")
            .input_type(InputType::Search)
            .value(&state.search)
            .on_input(|value| Message::Input(InputMessage::SearchChanged(value))),
    ]
    .spacing(15);

    // Filled + Pill combination
    let combo_title = text("Filled + Pill").size(24);
    let combo_input = Input::new("Search...")
        .filled(true)
        .pill(true)
        .input_type(InputType::Search)
        .value(&state.combo)
        .on_input(|value| Message::Input(InputMessage::ComboChanged(value)));

    // Clearable
    let clearable_title = text("Clearable").size(24);
    let clearable_desc = text("Inputs can have a clear button to quickly remove the value").size(14);
    let clearable_input = Input::new("Type something to see clear button")
        .label("Clearable Input")
        .value(&state.basic)
        .clearable(true)
        .on_input(|value| Message::Input(InputMessage::BasicChanged(value)))
        .on_clear(Message::Input(InputMessage::BasicChanged(String::new())));

    // Password Toggle
    let password_toggle_title = text("Password Toggle").size(24);
    let password_toggle_desc = text("Password inputs can have a toggle to show/hide the password").size(14);
    let password_toggle_input = Input::new("Enter your password")
        .label("Password with Toggle")
        .input_type(InputType::Password)
        .password_toggle(true)
        .value(&state.password_type)
        .on_input(|value| Message::Input(InputMessage::PasswordTypeChanged(value)));

    // Min/Max Length
    let length_title = text("Length Constraints").size(24);
    let length_column = column![
        Input::new("Min 3 characters")
            .label("Username")
            .min_length(3)
            .value(&state.name)
            .on_input(|value| Message::Input(InputMessage::NameChanged(value))),
        Input::new("Max 10 characters")
            .label("Short Code")
            .max_length(10)
            .value(&state.small)
            .on_input(|value| Message::Input(InputMessage::SmallChanged(value))),
        Input::new("Between 8-20 characters")
            .label("Password")
            .input_type(InputType::Password)
            .min_length(8)
            .max_length(20)
            .password_toggle(true)
            .value(&state.password)
            .on_input(|value| Message::Input(InputMessage::PasswordChanged(value))),
    ]
    .spacing(15);

    // Required Fields
    let required_title = text("Required Fields").size(24);
    let required_desc = text("Required fields are marked with an asterisk (*)").size(14);
    let required_column = column![
        Input::new("Enter your email")
            .label("Email")
            .input_type(InputType::Email)
            .required(true)
            .value(&state.email_type)
            .on_input(|value| Message::Input(InputMessage::EmailTypeChanged(value))),
        Input::new("Enter your name")
            .label("Full Name")
            .required(true)
            .value(&state.first_name)
            .on_input(|value| Message::Input(InputMessage::FirstNameChanged(value))),
    ]
    .spacing(15);

    // Combined Features
    let combined_title = text("Combined Features").size(24);
    let combined_desc = text("Inputs can combine multiple features").size(14);
    let combined_input = Input::new("Search...")
        .label("Advanced Search")
        .filled(true)
        .pill(true)
        .clearable(true)
        .max_length(50)
        .help_text("Search with up to 50 characters")
        .value(&state.search)
        .on_input(|value| Message::Input(InputMessage::SearchChanged(value)))
        .on_clear(Message::Input(InputMessage::SearchChanged(String::new())));

    column![
        title,
        description,
        basic_title,
        basic_input,
        label_title,
        label_input,
        value_title,
        value_input,
        help_text_title,
        help_text_input,
        sizes_title,
        sizes_column,
        types_title,
        types_column,
        filled_title,
        filled_input,
        pill_title,
        pill_row,
        disabled_title,
        disabled_input,
        readonly_title,
        readonly_input,
        placeholders_title,
        placeholders_column,
        combo_title,
        combo_input,
        clearable_title,
        clearable_desc,
        clearable_input,
        password_toggle_title,
        password_toggle_desc,
        password_toggle_input,
        length_title,
        length_column,
        required_title,
        required_desc,
        required_column,
        combined_title,
        combined_desc,
        combined_input,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
