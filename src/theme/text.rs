use iced_widget::text;

use crate::theme::Theme;

pub struct TextStyleClass {
    color: Option<iced::Color>,
}

impl text::Catalog for Theme {
    type Class<'a> = TextStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        TextStyleClass { color: None }
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        text::Style { color: class.color }
    }
}
