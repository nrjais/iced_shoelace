use iced_widget::{Row, column, text};

use crate::components::scrollable;
use crate::theme::ScrollableClass;
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Scrollable Styles").size(32);
    let description = text("Different scrollable styles using Shoelace tokens").size(14);

    // Default scrollable
    let default_scrollable = {
        let content = column![
            text("Default Scrollable").size(16),
            text("Visible rail with medium scroller").size(12),
            text("Line 1"),
            text("Line 2"),
            text("Line 3"),
            text("Line 4"),
            text("Line 5"),
            text("Line 6"),
            text("Line 7"),
            text("Line 8"),
        ]
        .spacing(5)
        .padding(10);

        scrollable(content)
            .class(ScrollableClass::Default)
            .height(150)
            .width(300)
    };

    // Subtle scrollable
    let subtle_scrollable = {
        let content = column![
            text("Subtle Scrollable").size(16),
            text("Rail appears on hover").size(12),
            text("Line 1"),
            text("Line 2"),
            text("Line 3"),
            text("Line 4"),
            text("Line 5"),
            text("Line 6"),
            text("Line 7"),
            text("Line 8"),
        ]
        .spacing(5)
        .padding(10);

        scrollable(content)
            .class(ScrollableClass::Subtle)
            .height(150)
            .width(300)
    };

    // Comparison row
    let comparison_title = text("Side by Side Comparison").size(24);
    let scrollable_row =
        Row::with_children([default_scrollable.into(), subtle_scrollable.into()]).spacing(40);

    // Usage notes
    let notes_title = text("Usage Notes").size(24);
    let notes = column![
        text("• Default scrollable: Best for content areas where scrolling is expected"),
        text("• Subtle scrollable: Best for minimal UI where scrollbars should be unobtrusive"),
        text("• Both styles support custom width and height"),
        text("• Scrollbars automatically appear when content overflows"),
    ]
    .spacing(10);

    column![
        title,
        description,
        comparison_title,
        scrollable_row,
        notes_title,
        notes,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
