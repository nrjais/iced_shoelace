use iced::{Color, widget::rule};

use crate::theme::Theme;

/// Style class for rule (divider) widgets
#[derive(Debug, Clone, Copy, Default)]
pub enum RuleStyleClass {
    #[default]
    Default,
    Custom(Color),
}

// Implement rule Catalog trait
impl rule::Catalog for Theme {
    type Class<'a> = RuleStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        RuleStyleClass::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> rule::Style {
        let tokens = self.tokens();

        match class {
            RuleStyleClass::Default => {
                // Shoelace divider default color is neutral-300
                rule::Style {
                    color: tokens.neutral.c300,
                    radius: 0.0.into(),
                    fill_mode: rule::FillMode::Full,
                    snap: false,
                }
            }
            RuleStyleClass::Custom(color) => {
                rule::Style {
                    color: *color,
                    radius: 0.0.into(),
                    fill_mode: rule::FillMode::Full,
                    snap: false,
                }
            }
        }
    }
}
