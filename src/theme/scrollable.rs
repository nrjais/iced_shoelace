use iced_widget::{container, scrollable};

use crate::theme::Theme;

/// Scrollbar style class
#[derive(Debug, Clone, Copy, Default)]
pub enum ScrollableClass {
    #[default]
    Default,
    /// A subtle scrollbar that appears on hover
    Subtle,
}

impl scrollable::Catalog for Theme {
    type Class<'a> = ScrollableClass;

    fn default<'a>() -> Self::Class<'a> {
        ScrollableClass::Default
    }

    fn style(&self, class: &Self::Class<'_>, status: scrollable::Status) -> scrollable::Style {
        let tokens = self.tokens();

        let is_active = matches!(
            status,
            scrollable::Status::Hovered { .. } | scrollable::Status::Dragged { .. }
        );

        // Container style - no special styling needed
        let container = container::Style {
            background: None,
            border: iced::Border::default(),
            text_color: None,
            shadow: iced::Shadow::default(),
            snap: false,
        };

        // Rail and scroller styling based on class
        let (rail_bg, scroller_color, scroller_border) = match class {
            ScrollableClass::Default => {
                // Default style - visible rail with rounded scroller
                let rail_bg = if is_active {
                    Some(iced::Background::Color(tokens.neutral.c100))
                } else {
                    Some(iced::Background::Color(tokens.neutral.c50))
                };

                let scroller_color = if is_active {
                    tokens.neutral.c600
                } else {
                    tokens.neutral.c400
                };

                (
                    rail_bg,
                    scroller_color,
                    iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 2.0,
                        radius: tokens.border_radius.large.into(),
                    },
                )
            }
            ScrollableClass::Subtle => {
                // Subtle style - rail appears only on hover
                let rail_bg = if is_active {
                    Some(iced::Background::Color(tokens.neutral.c100))
                } else {
                    Some(iced::Background::Color(tokens.neutral.c50))
                };

                let scroller_color = if is_active {
                    tokens.neutral.c600
                } else {
                    iced::Color::TRANSPARENT
                };

                (
                    rail_bg,
                    scroller_color,
                    iced::Border {
                        color: iced::Color::TRANSPARENT,
                        width: 2.0,
                        radius: tokens.border_radius.large.into(),
                    },
                )
            }
        };

        let width = if is_active { 0.0 } else { 2.0 };

        scrollable::Style {
            container,
            vertical_rail: scrollable::Rail {
                background: rail_bg,
                border: iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width,
                    radius: tokens.border_radius.large.into(),
                },
                scroller: scrollable::Scroller {
                    color: scroller_color,
                    border: scroller_border,
                },
            },
            horizontal_rail: scrollable::Rail {
                background: rail_bg,
                border: iced::Border {
                    color: iced::Color::TRANSPARENT,
                    width,
                    radius: tokens.border_radius.large.into(),
                },
                scroller: scrollable::Scroller {
                    color: scroller_color,
                    border: scroller_border,
                },
            },
            gap: Some(iced::Background::Color(iced::Color::TRANSPARENT)),
        }
    }
}
