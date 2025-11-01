use crate::{
    Element,
    theme::{
        button::ButtonStyleClass,
        pallete::{ColorToken, ColorValue, ColorVariant},
        sizes::{BORDER_RADIUS, FONT_SIZE, LINE_HEIGHT, SPACING},
        text::TextStyleClass,
    },
};
use iced::{
    Alignment,
    border::Radius,
    widget::{row, text},
};
use iced_core::Length;

/// A Shoelace-style breadcrumb component for iced
///
/// This component implements the features from Shoelace's breadcrumb component:
/// - Hierarchical navigation path display
/// - Separators between items
/// - Support for clickable and non-clickable items
/// - Custom separator support
pub struct Breadcrumb<Message> {
    items: Vec<BreadcrumbItem<Message>>,
    separator: String,
}

/// A single item in a breadcrumb trail
pub struct BreadcrumbItem<Message> {
    label: String,
    on_press: Option<Message>,
}

impl<Message> BreadcrumbItem<Message> {
    /// Creates a new breadcrumb item with the given label
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_press: None,
        }
    }

    /// Sets the message that will be produced when the item is clicked
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

impl<Message> Breadcrumb<Message>
where
    Message: Clone,
{
    /// Creates a new breadcrumb with the given items
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            separator: "/".to_string(),
        }
    }

    /// Adds an item to the breadcrumb
    pub fn push(mut self, item: BreadcrumbItem<Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Sets a custom separator (default is "/")
    pub fn separator(mut self, separator: impl Into<String>) -> Self {
        self.separator = separator.into();
        self
    }
}

impl<Message> Default for Breadcrumb<Message>
where
    Message: Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: 'a> From<Breadcrumb<Message>> for Element<'a, Message>
where
    Message: Clone,
{
    fn from(breadcrumb: Breadcrumb<Message>) -> Self {
        let separator = breadcrumb.separator;
        let items_count = breadcrumb.items.len();

        let mut breadcrumb_row = row([]).spacing(SPACING.x2_small).align_y(Alignment::Center);

        for (idx, item) in breadcrumb.items.into_iter().enumerate() {
            let is_last = idx == items_count - 1;
            let label = item.label;
            let separator_str = separator.clone();

            // Create the label element
            let label_element = if let Some(message) = item.on_press {
                // Clickable item (link) - use button styled as link
                let button_style = ButtonStyleClass {
                    variant: crate::theme::button::ButtonVariant::Text,
                    outline: false,
                    border_radius: Radius::from(BORDER_RADIUS.medium),
                    disabled: false,
                };

                let button = iced::widget::button(
                    text(label)
                        .size(FONT_SIZE.medium)
                        .line_height(LINE_HEIGHT.normal)
                        .class(TextStyleClass {
                            color: Some(ColorToken::new(ColorVariant::Sky, ColorValue::C600)),
                        }),
                )
                .padding([0., SPACING.x_small])
                .class(button_style)
                .on_press(message);

                Element::from(button)
            } else {
                // Non-clickable item (current page)
                Element::from(
                    text(label)
                        .size(FONT_SIZE.medium)
                        .line_height(LINE_HEIGHT.normal)
                        .class(TextStyleClass {
                            color: Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C600)),
                        }),
                )
            };

            breadcrumb_row = breadcrumb_row.push(label_element);

            // Add separator if not the last item
            if !is_last {
                let separator_element = text(separator_str)
                    .size(FONT_SIZE.medium)
                    .line_height(LINE_HEIGHT.normal)
                    .class(TextStyleClass {
                        color: Some(ColorToken::new(ColorVariant::Neutral, ColorValue::C400)),
                    });

                breadcrumb_row = breadcrumb_row.push(separator_element);
            }
        }

        Element::from(breadcrumb_row.width(Length::Shrink).height(Length::Shrink))
    }
}
