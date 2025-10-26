use crate::theme::Theme;

pub mod button;
pub mod hovered;
pub mod scrollable;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

pub type ElementFn<'a, Arg, Message> = Box<dyn Fn(Arg) -> Element<'a, Message> + 'a>;

pub use hovered::{Hovered, hovered};
pub use scrollable::{Direction, scrollable, scrollable_with};
