use iced::{Background, widget::container};

use crate::theme::{
    Theme,
    pallete::{ColorToken, ColorValue, ColorVariant},
};

#[derive(Debug, Clone, Copy)]
pub struct ContainerStyleClass {
    background: Option<ColorToken>,
    text_color: Option<ColorToken>,
    border: iced::Border,
    shadow: iced::Shadow,
    snap: bool,
}

// Implement container Catalog trait
impl container::Catalog for Theme {
    type Class<'a> = ContainerStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerStyleClass {
            background: Some(ColorToken::new(ColorVariant::NeutralBase, ColorValue::C50)),
            text_color: None,
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            snap: true,
        }
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        let background = class
            .background
            .map(|color| color.get_color(self.tokens()))
            .map(Background::Color);
        let text_color = class.text_color.map(|color| color.get_color(self.tokens()));

        container::Style {
            background,
            text_color,
            border: class.border,
            shadow: class.shadow,
            snap: class.snap,
        }
    }
}
