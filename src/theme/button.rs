use iced::{Background, border::Radius};
use iced_widget::button;

use crate::theme::Theme;

/// Button variant types matching Shoelace design system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Primary,
    Success,
    Neutral,
    Warning,
    Danger,
    Text,
}

/// Button size options matching Shoelace design system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

/// Style class for custom button styling
#[derive(Debug, Clone, Copy)]
pub struct ButtonStyleClass {
    pub variant: ButtonVariant,
    pub outline: bool,
    pub border_radius: Radius,
    pub disabled: bool,
}

// Implement button Catalog trait
impl button::Catalog for Theme {
    type Class<'a> = ButtonStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonStyleClass {
            variant: ButtonVariant::Default,
            outline: false,
            border_radius: Radius::from(4.0),
            disabled: false,
        }
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let tokens = self.tokens();
        let is_hovered = matches!(status, button::Status::Hovered);
        let is_pressed = matches!(status, button::Status::Pressed);
        let is_disabled = class.disabled;

        // Determine the base color scale based on variant
        let color_scale = match class.variant {
            ButtonVariant::Default => tokens.neutral,
            ButtonVariant::Primary => tokens.primary,
            ButtonVariant::Success => tokens.success,
            ButtonVariant::Neutral => tokens.neutral,
            ButtonVariant::Warning => tokens.warning,
            ButtonVariant::Danger => tokens.danger,
            ButtonVariant::Text => tokens.primary,
        };

        let (background, text_color, border_color, shadow_color) = if is_disabled {
            // Disabled state - use neutral 300/400 for muted appearance
            let (bg, border) = if class.variant == ButtonVariant::Text || class.outline {
                (iced::Color::TRANSPARENT, tokens.neutral.c300)
            } else {
                (tokens.neutral.c300, tokens.neutral.c300)
            };

            let text = tokens.neutral.c500;

            (bg, text, border, iced::Color::TRANSPARENT)
        } else if class.outline {
            // Outline buttons - Shoelace style with color-600 border/text
            let bg = if is_pressed {
                color_scale.c700 // Darker background on press
            } else if is_hovered {
                color_scale.c50 // Light background on hover
            } else {
                iced::Color::TRANSPARENT
            };

            let text = if is_pressed {
                tokens.neutral_0 // White text when pressed
            } else {
                color_scale.c600 // Primary color for text
            };

            let border = color_scale.c600; // Border matches text color

            (bg, text, border, iced::Color::TRANSPARENT)
        } else if class.variant == ButtonVariant::Text {
            // Text buttons - minimal style with hover background
            let bg = if is_pressed {
                color_scale.c100
            } else if is_hovered {
                color_scale.c50
            } else {
                iced::Color::TRANSPARENT
            };

            (
                bg,
                color_scale.c600,
                iced::Color::TRANSPARENT,
                iced::Color::TRANSPARENT,
            )
        } else if class.variant == ButtonVariant::Default {
            // Default (neutral) variant - Shoelace uses neutral-0 background with border
            let bg = if is_pressed {
                tokens.neutral.c100
            } else if is_hovered {
                tokens.neutral.c50
            } else {
                tokens.neutral_0
            };

            let text = tokens.neutral.c700;
            let border = tokens.neutral.c300;
            let shadow = iced::Color::from_rgba8(0, 0, 0, 0.05);

            (bg, text, border, shadow)
        } else {
            // Filled buttons - Shoelace uses color-600 as base
            let bg = if is_pressed {
                color_scale.c700 // Darker on press
            } else if is_hovered {
                color_scale.c500 // Lighter on hover
            } else {
                color_scale.c600 // Base color
            };

            let shadow = iced::Color::from_rgba8(0, 0, 0, 0.08);

            (
                bg,
                tokens.neutral_0, // White text on colored backgrounds
                bg,
                shadow,
            )
        };

        // Apply shadow for filled buttons
        let shadow = if !is_disabled && class.variant != ButtonVariant::Text && !class.outline {
            iced::Shadow {
                color: shadow_color,
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 2.0,
            }
        } else {
            iced::Shadow {
                color: iced::Color::TRANSPARENT,
                offset: iced::Vector::ZERO,
                blur_radius: 0.0,
            }
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: iced::Border {
                color: border_color,
                width: if class.outline || class.variant == ButtonVariant::Default {
                    1.0
                } else {
                    0.0
                },
                radius: class.border_radius,
            },
            shadow,
            snap: false,
        }
    }
}
