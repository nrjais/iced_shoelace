use iced::widget::{
    self, Scrollable,
    scrollable::{self, Scrollbar},
};

use crate::{
    components::Element,
    theme::{Theme, sizes::SPACING},
};

pub enum Direction {
    Vertical,
    Horizontal,
    Both,
}

pub fn scrollable<'a, Message>(
    base: impl Into<Element<'a, Message>>,
) -> Scrollable<'a, Message, Theme> {
    scrollable_with(base, Direction::Vertical)
}

pub fn scrollable_with<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    direction: Direction,
) -> Scrollable<'a, Message, Theme> {
    let scrollbar_width = SPACING.medium; // 12.0

    let scrollbar: Scrollbar = Scrollbar::default()
        .spacing(0)
        .width(scrollbar_width)
        .scroller_width(scrollbar_width);

    let direction = match direction {
        Direction::Vertical => scrollable::Direction::Vertical(scrollbar),
        Direction::Horizontal => scrollable::Direction::Horizontal(scrollbar),
        Direction::Both => scrollable::Direction::Both {
            vertical: scrollbar,
            horizontal: scrollbar,
        },
    };

    widget::scrollable(base).direction(direction)
}
