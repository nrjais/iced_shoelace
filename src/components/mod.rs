use crate::theme::Theme;

pub mod button;
pub mod button_group;
pub mod hovered;
pub mod scrollable;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

pub type ElementFn<'a, Arg, Message> = Box<dyn Fn(Arg) -> Element<'a, Message> + 'a>;

pub use button_group::{ButtonGroup, button_group, button_group_with};
pub use hovered::{Hovered, hovered};
pub use scrollable::{Direction, scrollable, scrollable_with};
