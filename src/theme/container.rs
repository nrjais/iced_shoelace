use iced::widget::container;

use crate::theme::Theme;

pub struct ContainerStyleClass {}

// Implement container Catalog trait
impl container::Catalog for Theme {
    type Class<'a> = ContainerStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerStyleClass {}
    }

    fn style(&self, _class: &Self::Class<'_>) -> container::Style {
        container::Style {
            background: Some(iced::Background::Color(self.tokens().neutral_0)),
            text_color: Some(self.tokens().gray.c900),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            snap: true,
        }
    }
}
