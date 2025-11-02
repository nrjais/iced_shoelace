use iced::{Background, border::Radius};
use iced_widget::text_input;

use super::sizes::BORDER_RADIUS;
use crate::components::input::InputSize;
use crate::theme::Theme;

/// Style class for custom input styling matching Shoelace design system
///
/// Shoelace input styling reference:
/// - Border: 1px solid neutral-300 (default), neutral-400 (hover), primary-500 (focus)
/// - Background: neutral-0 (standard), neutral-50 (filled)
/// - Text: neutral-900
/// - Placeholder: neutral-500
/// - Selection: primary-100
/// - Disabled: background neutral-50, text neutral-400, border neutral-300
#[derive(Debug, Clone, Copy)]
pub struct InputStyleClass {
    pub size: InputSize,
    pub disabled: bool,
    pub filled: bool,
    pub pill: bool,
}

// Implement text_input Catalog trait
impl text_input::Catalog for Theme {
    type Class<'a> = InputStyleClass;

    fn default<'a>() -> Self::Class<'a> {
        InputStyleClass {
            size: InputSize::Medium,
            disabled: false,
            filled: false,
            pill: false,
        }
    }

    fn style(&self, class: &Self::Class<'_>, status: text_input::Status) -> text_input::Style {
        let tokens = self.tokens();
        let is_focused = matches!(status, text_input::Status::Focused { .. });
        let is_hovered = matches!(status, text_input::Status::Hovered);
        let is_disabled = class.disabled;

        // Determine border radius based on pill and size
        // Pill style uses very large border radius to create fully rounded ends
        let border_radius = if class.pill {
            Radius::from(999.0)
        } else {
            // Standard border radius based on size (Shoelace uses medium=4px by default)
            match class.size {
                InputSize::Small => Radius::from(BORDER_RADIUS.small),   // 3px
                InputSize::Medium => Radius::from(BORDER_RADIUS.medium), // 4px
                InputSize::Large => Radius::from(BORDER_RADIUS.large),   // 8px
            }
        };

        // Shoelace-style color mapping based on state
        let (background, text_color, border_color, placeholder_color) = if is_disabled {
            // Disabled state - neutral-50 background, muted text and borders
            let bg = tokens.neutral.c50;

            (
                bg,
                tokens.neutral.c400,  // Muted text color
                tokens.neutral.c300,  // Light border
                tokens.neutral.c400,  // Muted placeholder
            )
        } else if class.filled {
            // Filled variant - neutral-50 background by default, neutral-100 on hover/focus
            let bg = if is_focused || is_hovered {
                tokens.neutral.c100
            } else {
                tokens.neutral.c50
            };

            let border = if is_focused {
                tokens.primary.c500   // Primary color on focus
            } else if is_hovered {
                tokens.neutral.c400   // Slightly darker on hover
            } else {
                tokens.neutral.c300   // Default neutral border
            };

            (
                bg,
                tokens.neutral.c900,  // Dark text for contrast
                border,
                tokens.neutral.c500,  // Medium neutral for placeholder
            )
        } else {
            // Standard variant - white (neutral-0) background with border states
            let bg = tokens.neutral_0;

            let border = if is_focused {
                tokens.primary.c500   // Primary blue on focus
            } else if is_hovered {
                tokens.neutral.c400   // Slightly darker on hover
            } else {
                tokens.neutral.c300   // Default light neutral border
            };

            (
                bg,
                tokens.neutral.c900,  // Dark text
                border,
                tokens.neutral.c500,  // Medium neutral placeholder
            )
        };

        // Selection color - light primary for text selection
        let selection_color = tokens.primary.c100;

        text_input::Style {
            background: Background::Color(background),
            border: iced::Border {
                color: border_color,
                width: 1.0,  // Shoelace uses 1px border
                radius: border_radius,
            },
            icon: text_color,
            placeholder: placeholder_color,
            value: text_color,
            selection: selection_color,
        }
    }
}
