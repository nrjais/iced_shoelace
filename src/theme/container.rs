use iced::{Background, Border, Color, Shadow, widget::container};

use crate::theme::{
    Theme,
    badge::BadgeVariant,
    pallete::{ColorToken, ColorValue, ColorVariant},
    sizes::BORDER_RADIUS,
};

#[derive(Debug, Clone, Copy, Default)]
pub enum ContainerStyleClass {
    #[default]
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
    /// Dialog - white background with border, shadow, and rounded corners (always light)
    Dialog,
    /// Dialog header - transparent background with no border (separation via padding)
    DialogHeader,
    /// Dialog footer - transparent background with no border (separation via padding)
    DialogFooter,
    Custom {
        background: Option<ColorToken>,
        text_color: Option<ColorToken>,
        border: Border,
        shadow: Shadow,
        snap: bool,
    },
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
                // Default container background
                // Light theme: light gray (neutral-50) for subtle contrast with white cards
                // Dark theme: darkest (neutral-0) for maximum contrast with lighter cards
                let background = match self {
                    crate::theme::Theme::Light => {
                        ColorToken::new(ColorVariant::Neutral, ColorValue::C50).get_color(tokens)
                    }
                    crate::theme::Theme::Dark => tokens.neutral_0,
                };

                container::Style {
                    background: Some(Background::Color(background)),
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
                // Uses --sl-panel-border-color (neutral-200), --sl-border-radius-medium, --sl-shadow-x-small
                // See: https://github.com/shoelace-style/shoelace/blob/next/src/components/card/card.styles.ts
                // Light theme: white background (neutral-0) on light gray page (neutral-50)
                // Dark theme: lighter panel (neutral-100) on dark page (neutral-0/50)
                let (background, text_color, border_color) = match self {
                    crate::theme::Theme::Light => {
                        (tokens.neutral_0, tokens.neutral.c700, tokens.neutral.c200)
                    }
                    crate::theme::Theme::Dark => (
                        tokens.neutral.c100, // Slightly lighter than page background
                        tokens.neutral.c900,
                        tokens.neutral.c200,
                    ),
                };

                container::Style {
                    background: Some(Background::Color(background)),
                    text_color: Some(text_color),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: BORDER_RADIUS.medium.into(),
                    },
                    shadow: Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: iced::Vector::new(0.0, 1.0),
                        blur_radius: 3.0,
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
            ContainerStyleClass::Dialog => {
                // Dialog styling matching Shoelace design
                // Uses --sl-panel-background-color (neutral-0), --sl-panel-border-color (neutral-200)
                // --sl-border-radius-medium, --sl-shadow-x-large
                // See: https://github.com/shoelace-style/shoelace/blob/next/src/components/dialog/dialog.styles.ts
                // Shoelace elevation token --sl-shadow-x-large: 0 8px 25px -8px hsl(240 3.8% 46.1% / 15%)
                container::Style {
                    background: Some(Background::Color(tokens.neutral_0)),
                    text_color: Some(tokens.neutral.c700),
                    border: Border {
                        color: tokens.neutral.c200,
                        width: 1.,
                        radius: BORDER_RADIUS.medium.into(),
                    },
                    shadow: Shadow {
                        // Shoelace's x-large shadow: more prominent and softer than cards
                        // --sl-shadow-x-large: 0 8px 25px -8px with 15% opacity
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                        offset: iced::Vector::new(0.0, 8.0),
                        blur_radius: 25.0,
                    },
                    snap: false,
                }
            }
            ContainerStyleClass::DialogHeader => {
                // Dialog header - no border (Iced doesn't support bottom-only borders)
                // Separation from body is achieved through padding
                container::Style {
                    background: None,
                    text_color: None,
                    border: Border::default(),
                    shadow: Shadow::default(),
                    snap: false,
                }
            }
            ContainerStyleClass::DialogFooter => {
                // Dialog footer - no border (Iced doesn't support top-only borders)
                // Separation from body is achieved through padding
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
