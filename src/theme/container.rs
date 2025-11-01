use iced::{Background, Border, Color, Shadow, widget::container};

use crate::theme::{
    Theme,
    pallete::{ColorToken, ColorValue, ColorVariant},
    sizes::BORDER_RADIUS,
};

#[derive(Debug, Clone, Copy)]
pub enum ContainerStyleClass {
    Default,
    Tooltip,
    /// Button group container - transparent with no styling
    ButtonGroup,
    Custom {
        background: Option<ColorToken>,
        text_color: Option<ColorToken>,
        border: Border,
        shadow: Shadow,
        snap: bool,
    },
}

impl Default for ContainerStyleClass {
    fn default() -> Self {
        Self::Default
    }
}

// Implement container Catalog trait
impl container::Catalog for Theme {
    type Class<'a> = ContainerStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        ContainerStyleClass::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> container::Style {
        let tokens = self.tokens();

        match class {
            ContainerStyleClass::Default => {
                container::Style {
                    background: Some(Background::Color(
                        ColorToken::new(ColorVariant::NeutralBase, ColorValue::C50).get_color(tokens)
                    )),
                    text_color: None,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: true,
                }
            }
            ContainerStyleClass::Tooltip => {
                // Shoelace tooltip styling
                // Dark background with white text, medium border radius, and subtle shadow
                container::Style {
                    background: Some(Background::Color(tokens.neutral.c800)),
                    text_color: Some(tokens.neutral_0),
                    border: Border {
                        radius: BORDER_RADIUS.medium.into(),
                        ..Default::default()
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 8.0,
                    },
                    snap: false,
                }
            }
            ContainerStyleClass::ButtonGroup => {
                // Button group container - transparent with no styling
                // Matches Shoelace's button-group ::part(base) which is inline-flex
                container::Style {
                    background: None,
                    text_color: None,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ContainerStyleClass::Custom {
                background,
                text_color,
                border,
                shadow,
                snap,
            } => {
                let bg = background
                    .map(|color| color.get_color(tokens))
                    .map(Background::Color);
                let txt_color = text_color.map(|color| color.get_color(tokens));

                container::Style {
                    background: bg,
                    text_color: txt_color,
                    border: *border,
                    shadow: *shadow,
                    snap: *snap,
                }
            }
        }
    }
}
