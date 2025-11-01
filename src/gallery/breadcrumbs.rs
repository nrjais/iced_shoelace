use iced_widget::{column, text};

use crate::components::{Breadcrumb, BreadcrumbItem};
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Breadcrumbs").size(32);
    let description = text("Use breadcrumbs to show navigation hierarchy").size(14);

    // Basic breadcrumb
    let basic_title = text("Basic Breadcrumb").size(24);
    let basic_desc = text("A simple breadcrumb trail").size(14);
    let basic_breadcrumb = Breadcrumb::new()
        .push(BreadcrumbItem::new("Home").on_press(Message::ButtonPressed("Home".into())))
        .push(BreadcrumbItem::new("Clothing").on_press(Message::ButtonPressed("Clothing".into())))
        .push(BreadcrumbItem::new("Women's").on_press(Message::ButtonPressed("Women's".into())))
        .push(BreadcrumbItem::new("Tops"));

    // Breadcrumb with different separator
    let separator_title = text("Custom Separator").size(24);
    let separator_desc = text("Using a different separator character").size(14);
    let separator_breadcrumb = Breadcrumb::new()
        .separator("›")
        .push(BreadcrumbItem::new("Home").on_press(Message::ButtonPressed("Home".into())))
        .push(
            BreadcrumbItem::new("Electronics")
                .on_press(Message::ButtonPressed("Electronics".into())),
        )
        .push(BreadcrumbItem::new("Computers").on_press(Message::ButtonPressed("Computers".into())))
        .push(BreadcrumbItem::new("Laptops"));

    // Breadcrumb with arrow separator
    let arrow_title = text("Arrow Separator").size(24);
    let arrow_desc = text("Using arrow as separator").size(14);
    let arrow_breadcrumb = Breadcrumb::new()
        .separator("→")
        .push(BreadcrumbItem::new("Dashboard").on_press(Message::ButtonPressed("Dashboard".into())))
        .push(BreadcrumbItem::new("Projects").on_press(Message::ButtonPressed("Projects".into())))
        .push(BreadcrumbItem::new("Current Project"));

    // Breadcrumb with bullet separator
    let bullet_title = text("Bullet Separator").size(24);
    let bullet_desc = text("Using bullet as separator").size(14);
    let bullet_breadcrumb = Breadcrumb::new()
        .separator("•")
        .push(BreadcrumbItem::new("Root").on_press(Message::ButtonPressed("Root".into())))
        .push(BreadcrumbItem::new("Documents").on_press(Message::ButtonPressed("Documents".into())))
        .push(BreadcrumbItem::new("Projects").on_press(Message::ButtonPressed("Projects".into())))
        .push(BreadcrumbItem::new("2024"));

    column![
        title,
        description,
        basic_title,
        basic_desc,
        basic_breadcrumb,
        separator_title,
        separator_desc,
        separator_breadcrumb,
        arrow_title,
        arrow_desc,
        arrow_breadcrumb,
        bullet_title,
        bullet_desc,
        bullet_breadcrumb,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
