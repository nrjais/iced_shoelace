use iced::{Background, Border, Color, Shadow, widget::container};

use crate::theme::{
    Theme,
    badge::BadgeVariant,
    pallete::{ColorToken, ColorValue, ColorVariant},
    sizes::BORDER_RADIUS,
};

#[derive(Debug, Clone, Copy)]
pub enum ContainerStyleClass {
    Default,
    Tooltip,
    /// Button group container - transparent with no styling
    ButtonGroup,
    /// Badge container with variant styling
    Badge {
        variant: BadgeVariant,
        border_radius: f32,
        pulse: bool,
    },
    /// Card container - white background with border, shadow, and rounded corners
    Card,
    /// Card header - transparent background
    CardHeader,
    /// Card content - transparent background
    CardContent,
    /// Card footer - transparent background
    CardFooter,
    /// Card image - transparent background with top border radius
    CardImage,
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
            ContainerStyleClass::Default => container::Style {
                background: Some(Background::Color(
                    ColorToken::new(ColorVariant::NeutralBase, ColorValue::C50).get_color(tokens),
                )),
                text_color: None,
                border: Border::default(),
                shadow: Shadow::default(),
                snap: true,
            },
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
            ContainerStyleClass::Badge {
                variant,
                border_radius,
                pulse: _pulse,
            } => {
                // Badge styling matching Shoelace design
                // Note: pulse animation is not implemented in the static style
                let (background, text_color) = match variant {
                    BadgeVariant::Primary => (tokens.primary.c600, tokens.neutral_0),
                    BadgeVariant::Success => (tokens.success.c600, tokens.neutral_0),
                    BadgeVariant::Neutral => (tokens.neutral.c600, tokens.neutral_0),
                    BadgeVariant::Warning => (tokens.warning.c600, tokens.neutral_0),
                    BadgeVariant::Danger => (tokens.danger.c600, tokens.neutral_0),
                };

                container::Style {
                    background: Some(Background::Color(background)),
                    text_color: Some(text_color),
                    border: Border {
                        color: background,
                        width: 0.0,
                        radius: (*border_radius).into(),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ContainerStyleClass::Card => {
                // Card styling matching Shoelace design
                // White/neutral background with subtle border, shadow, and rounded corners
                container::Style {
                    background: Some(Background::Color(tokens.neutral_0)),
                    text_color: Some(tokens.neutral.c900),
                    border: Border {
                        color: ColorToken::new(ColorVariant::Neutral, ColorValue::C200)
                            .get_color(tokens),
                        width: 1.0,
                        radius: BORDER_RADIUS.large.into(),
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 8.0,
                    },
                    snap: false,
                }
            }
            ContainerStyleClass::CardHeader => {
                // Card header - transparent background
                container::Style {
                    background: None,
                    text_color: None,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ContainerStyleClass::CardContent => {
                // Card content - transparent background
                container::Style {
                    background: None,
                    text_color: None,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ContainerStyleClass::CardFooter => {
                // Card footer - transparent background
                container::Style {
                    background: None,
                    text_color: None,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ContainerStyleClass::CardImage => {
                // Card image - transparent background with top border radius
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
