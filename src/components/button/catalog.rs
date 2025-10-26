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
            border_radius: 4.0,
            hovered: false,
            pressed: false,
        }
    }

    fn style(&self, class: &Self::Class<'_>, status: button::Status) -> button::Style {
        let tokens = self.tokens();
        let is_hovered = class.hovered || matches!(status, button::Status::Hovered);
        let is_pressed = class.pressed || matches!(status, button::Status::Pressed);

        // Determine the base color scale based on variant
        let color_scale = match class.variant {
            Variant::Default => tokens.neutral,
            Variant::Primary => tokens.primary,
            Variant::Success => tokens.success,
            Variant::Neutral => tokens.neutral,
            Variant::Warning => tokens.warning,
            Variant::Danger => tokens.danger,
            Variant::Text => tokens.neutral,
        };

        let (background, text_color, border_color, shadow_color) = if class.outline {
            // Outline style
            let border_color = if is_pressed {
                color_scale.c700
            } else if is_hovered {
                color_scale.c600
            } else {
                color_scale.c500
            };

            let text = if is_pressed {
                color_scale.c700
            } else if is_hovered {
                color_scale.c600
            } else {
                color_scale.c500
            };

            (
                tokens.neutral_0,
                text,
                border_color,
                iced::Color::TRANSPARENT,
            )
        } else if class.variant == Variant::Text {
            // Text style
            let text = if is_pressed {
                color_scale.c700
            } else if is_hovered {
                color_scale.c600
            } else {
                color_scale.c500
            };

            (
                iced::Color::TRANSPARENT,
                text,
                iced::Color::TRANSPARENT,
                iced::Color::TRANSPARENT,
            )
        } else {
            // Filled style
            let bg = if is_pressed {
                color_scale.c700
            } else if is_hovered {
                color_scale.c600
            } else {
                color_scale.c500
            };

            (
                bg,
                tokens.neutral_0,
                bg,
                iced::Color::from_rgba8(0, 0, 0, 0.1),
            )
        };

        let shadow = if class.variant != Variant::Text && !class.outline {
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
                width: if class.outline { 1.0 } else { 0.0 },
                radius: class.border_radius.into(),
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
