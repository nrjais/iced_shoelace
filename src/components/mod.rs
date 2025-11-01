use crate::theme::Theme;

pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod card;
pub mod checkbox;
pub mod divider;
pub mod dropdown;
pub mod hovered;
pub mod menu;
pub mod menu_item;
pub mod menu_label;
pub mod popup;
pub mod scrollable;
pub mod tooltip;

pub type Element<'a, Message> = iced::Element<'a, Message, Theme>;

pub type ElementFn<'a, Arg, Message> = Box<dyn Fn(Arg) -> Element<'a, Message> + 'a>;

pub use badge::Badge;
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use button_group::{ButtonGroup, button_group, button_group_with};
pub use card::Card;
pub use checkbox::{Checkbox, CheckboxSize, checkbox};
pub use divider::{Divider, divider};
pub use dropdown::{Dropdown, dropdown};
pub use hovered::{Hovered, hovered};
pub use menu::{Menu, menu, menu_with};
pub use menu_item::{MenuItem, MenuItemType, menu_item};
pub use menu_label::{MenuLabel, menu_label};
pub use popup::{Placement as PopupPlacement, Popup, popup};
pub use scrollable::{Direction, scrollable, scrollable_with};
pub use tooltip::{Placement as TooltipPlacement, Tooltip, tooltip};
