use iced_widget::{button, container, text};

use crate::{
    components::button::{ButtonStyleClass, Variant},
    theme::Theme,
};

// Implement button Catalog trait
impl button::Catalog for Theme {
    type Class<'a> = ButtonStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        ButtonStyleClass {
            variant: Variant::Default,
            outline: false,
            border_radius: iced::border::Radius::from(4.0),
            hovered: false,
            disabled: false,
        }
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let tokens = self.tokens();
        let is_hovered = class.hovered;
        let is_pressed = matches!(status, button::Status::Pressed);
        let is_disabled = class.disabled;

        // Determine the base color scale based on variant
        let color_scale = match class.variant {
            Variant::Default => tokens.neutral,
            Variant::Primary => tokens.primary,
            Variant::Success => tokens.success,
            Variant::Neutral => tokens.neutral,
            Variant::Warning => tokens.warning,
            Variant::Danger => tokens.danger,
            Variant::Text => tokens.primary,
        };

        let (background, text_color, border_color, shadow_color) = if is_disabled {
            // Disabled state - desaturated colors
            let (bg, border) = if class.variant == Variant::Text {
                (iced::Color::TRANSPARENT, color_scale.c100)
            } else if class.outline {
                (iced::Color::TRANSPARENT, color_scale.c100)
            } else {
                (color_scale.c100, iced::Color::TRANSPARENT)
            };

            let text = tokens.neutral.c400;

            (bg, text, border, iced::Color::TRANSPARENT)
        } else if class.outline {
            // Outline style - background changes on hover, border remains the same
            let bg = if is_pressed {
                color_scale.c600 // Slightly darker background on press
            } else if is_hovered {
                color_scale.c50 // Light background on hover
            } else {
                iced::Color::TRANSPARENT
            };

            let text = if is_pressed {
                tokens.neutral_0 // White text when pressed
            } else {
                color_scale.c600 // Colored text always
            };

            let border = color_scale.c600; // Border color stays consistent

            (bg, text, border, iced::Color::TRANSPARENT)
        } else if class.variant == Variant::Text {
            // Text style - subtle background on hover
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
        } else if class.variant == Variant::Default {
            // Default variant - white/light background with border and dark text
            let bg = if is_pressed {
                tokens.neutral.c50
            } else if is_hovered {
                tokens.neutral.c50
            } else {
                tokens.neutral_0
            };

            let text = tokens.neutral.c700;
            let border = tokens.neutral.c300;

            (bg, text, border, iced::Color::from_rgba8(0, 0, 0, 0.1))
        } else {
            // Filled style - darken on hover
            let bg = if is_pressed {
                color_scale.c500 // Darkest on press
            } else if is_hovered {
                color_scale.c400 // Darker on hover
            } else {
                color_scale.c500 // Base color
            };

            (
                bg,
                tokens.neutral_0,
                bg,
                iced::Color::from_rgba8(0, 0, 0, 0.1),
            )
        };

        let shadow = if !is_disabled && class.variant != Variant::Text && !class.outline {
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
            background: Some(iced::Background::Color(background)),
            text_color,
            border: iced::Border {
                color: border_color,
                width: if class.outline || class.variant == Variant::Default {
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

// Implement text Catalog trait
impl text::Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {
        ()
    }

    fn style(&self, _class: &Self::Class<'_>) -> text::Style {
        text::Style {
            color: Some(self.tokens().gray.c900),
        }
    }
}

// Implement container Catalog trait
impl container::Catalog for Theme {
    type Class<'a> = ();

    fn default<'a>() -> Self::Class<'a> {
        ()
    }

    fn style(&self, _class: &Self::Class<'_>) -> container::Style {
        container::Style {
            background: Some(iced::Background::Color(self.tokens().neutral_0)),
            text_color: Some(self.tokens().gray.c900),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
            snap: false,
        }
    }
}
