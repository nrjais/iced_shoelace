use iced_widget::checkbox;

use crate::theme::Theme;

/// Style class for custom checkbox styling
#[derive(Debug, Clone, Copy, Default)]
pub struct CheckboxStyleClass {
    pub is_checked: bool,
    pub is_indeterminate: bool,
    pub is_disabled: bool,
}

// Implement checkbox Catalog trait
impl checkbox::Catalog for Theme {
    type Class<'a> = CheckboxStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        CheckboxStyleClass {
            is_checked: false,
            is_indeterminate: false,
            is_disabled: false,
        }
    }

    fn style(&self, class: &Self::Class<'_>, status: checkbox::Status) -> checkbox::Style {
        let tokens = self.tokens();
        let is_hovered = matches!(status, checkbox::Status::Hovered { is_checked: _ });

        let (background, border_color, icon_color) = if class.is_disabled {
            // Disabled state
            (
                tokens.neutral.c100,
                tokens.neutral.c300,
                tokens.neutral.c400,
            )
        } else if class.is_checked || class.is_indeterminate {
            // Checked or indeterminate state
            let bg = if is_hovered {
                tokens.primary.c500
            } else {
                tokens.primary.c600
            };
            (bg, bg, tokens.neutral_0)
        } else {
            // Unchecked state
            let bg = tokens.neutral_0;
            let border = if is_hovered {
                tokens.primary.c500
            } else {
                tokens.neutral.c300
            };
            (bg, border, tokens.neutral_0)
        };

        checkbox::Style {
            background: iced::Background::Color(background),
            icon_color,
            border: iced::Border {
                color: border_color,
                width: 1.0,
                radius: 2.0.into(),
            },
            text_color: None,
        }
    }
}
