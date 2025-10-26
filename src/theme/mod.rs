use iced::theme;

use crate::theme::tokens::Tokens;

pub mod pallete;
pub mod scrollable;
pub mod tokens;
pub mod button;

pub use scrollable::ScrollableClass;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub fn tokens(&self) -> Tokens {
        match self {
            Theme::Dark => crate::theme::pallete::DARK,
            Theme::Light => crate::theme::pallete::LIGHT,
        }
    }
}

impl theme::Base for Theme {
    fn default(preference: theme::Mode) -> Self {
        match preference {
            theme::Mode::None => Theme::Dark,
            theme::Mode::Light => Theme::Light,
            theme::Mode::Dark => Theme::Dark,
        }
    }

    fn mode(&self) -> theme::Mode {
        match self {
            Theme::Light => theme::Mode::Light,
            Theme::Dark => theme::Mode::Dark,
        }
    }

    fn base(&self) -> theme::Style {
        theme::Style {
            background_color: self.tokens().neutral_0,
            text_color: self.tokens().gray.c900,
        }
    }

    fn palette(&self) -> Option<theme::Palette> {
        Some(theme::Palette {
            background: self.tokens().neutral_0,
            text: self.tokens().gray.c900,
            primary: self.tokens().primary.c500,
            success: self.tokens().success.c500,
            warning: self.tokens().warning.c500,
            danger: self.tokens().danger.c500,
        })
    }
}
