use std::cell::RefCell;

use crate::theme::Theme;

pub mod button;
pub mod button_group;
pub mod hovered;
pub mod scrollable;
pub mod tooltip;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

pub type ElementFn<'a, Arg, Message> = Box<dyn Fn(Arg) -> Element<'a, Message> + 'a>;

pub use button_group::{ButtonGroup, button_group, button_group_with};
pub use hovered::{Hovered, hovered};
pub use scrollable::{Direction, scrollable, scrollable_with};
pub use tooltip::{Placement as TooltipPlacement, Tooltip, tooltip};

pub struct ElementBox<'a, Message> {
    element: RefCell<Element<'a, Message>>,
}

impl<'a, Message> ElementBox<'a, Message> {
    pub fn new(element: impl Into<Element<'a, Message>>) -> Self {
        Self {
            element: RefCell::new(element.into()),
        }
    }

    pub fn take(&self) -> Element<'a, Message> {
        let empty = None::<Element<'a, Message>>.into();
        self.element.replace(empty)
    }
}
