use iced::alignment;
use iced_widget::{Row, column, text};

use crate::components::checkbox::{Checkbox, CheckboxSize};
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Checkboxes").size(32);
    let description = text("Allow users to toggle an option on or off").size(14);

    // Basic checkbox
    let basic_title = text("Basic").size(24);
    let basic_checkbox = Checkbox::new("Checkbox", false)
        .on_toggle(|checked| Message::CheckboxChanged("Basic".into(), checked));

    // Checked
    let checked_title = text("Checked").size(24);
    let checked_checkbox = Checkbox::new("Checked", true)
        .on_toggle(|checked| Message::CheckboxChanged("Checked".into(), checked));

    // Indeterminate
    let indeterminate_title = text("Indeterminate").size(24);
    let indeterminate_checkbox = Checkbox::new("Indeterminate", false)
        .indeterminate(true)
        .on_toggle(|checked| Message::CheckboxChanged("Indeterminate".into(), checked));

    // Disabled
    let disabled_title = text("Disabled").size(24);
    let disabled_row = Row::with_children([
        Checkbox::new("Disabled", false).disabled(true).into(),
        Checkbox::new("Disabled Checked", true)
            .disabled(true)
            .into(),
    ])
    .spacing(20);

    // Sizes
    let sizes_title = text("Sizes").size(24);
    let sizes_row = Row::with_children([
        Checkbox::new("Small", false)
            .size(CheckboxSize::Small)
            .on_toggle(|checked| Message::CheckboxChanged("Small".into(), checked))
            .into(),
        Checkbox::new("Medium", false)
            .size(CheckboxSize::Medium)
            .on_toggle(|checked| Message::CheckboxChanged("Medium".into(), checked))
            .into(),
        Checkbox::new("Large", false)
            .size(CheckboxSize::Large)
            .on_toggle(|checked| Message::CheckboxChanged("Large".into(), checked))
            .into(),
    ])
    .spacing(20)
    .align_y(alignment::Vertical::Center);

    // Help Text
    let help_text_title = text("Help Text").size(24);
    let help_text_checkbox = Checkbox::new("Label", false)
        .help_text("What should the user know about the checkbox?")
        .on_toggle(|checked| Message::CheckboxChanged("HelpText".into(), checked));

    // Different labels
    let labels_title = text("Different Labels").size(24);
    let labels_column = column![
        Checkbox::new("I agree to the terms and conditions", false)
            .on_toggle(|checked| Message::CheckboxChanged("Terms".into(), checked)),
        Checkbox::new("Subscribe to newsletter", false)
            .on_toggle(|checked| Message::CheckboxChanged("Newsletter".into(), checked)),
        Checkbox::new("Remember me", false)
            .on_toggle(|checked| Message::CheckboxChanged("Remember".into(), checked)),
    ]
    .spacing(15);

    column![
        title,
        description,
        basic_title,
        basic_checkbox,
        checked_title,
        checked_checkbox,
        indeterminate_title,
        indeterminate_checkbox,
        disabled_title,
        disabled_row,
        sizes_title,
        sizes_row,
        help_text_title,
        help_text_checkbox,
        labels_title,
        labels_column,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
