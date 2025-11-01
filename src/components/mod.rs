use crate::theme::Theme;

pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod card;
pub mod checkbox;
pub mod hovered;
pub mod scrollable;
pub mod tooltip;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

pub type ElementFn<'a, Arg, Message> = Box<dyn Fn(Arg) -> Element<'a, Message> + 'a>;

pub use badge::Badge;
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use button_group::{ButtonGroup, button_group, button_group_with};
pub use card::Card;
pub use checkbox::{Checkbox, CheckboxSize, checkbox};
pub use hovered::{Hovered, hovered};
pub use scrollable::{Direction, scrollable, scrollable_with};
pub use tooltip::{Placement as TooltipPlacement, Tooltip, tooltip};
