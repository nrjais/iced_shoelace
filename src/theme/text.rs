use iced_widget::text;

use crate::theme::{Theme, pallete::ColorToken};

pub struct TextStyleClass {
    color: Option<ColorToken>,
}

impl text::Catalog for Theme {
    type Class<'a> = TextStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        TextStyleClass { color: None }
    }

    fn style(&self, class: &Self::Class<'_>) -> text::Style {
        let color = class.color.map(|color| color.get_color(self.tokens()));
        text::Style { color }
    }
}
