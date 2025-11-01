use iced_widget::{Row, column, text};

use crate::components::button::Button;
use crate::components::Card;
use crate::theme::button::{ButtonSize, ButtonVariant};
use crate::{Element, Message};

pub fn page() -> Element<'static, Message> {
    let title = text("Cards").size(32);
    let description = text("Cards group related subjects in a container with optional header, footer, and image").size(14);

    // Basic Card
    let basic_title = text("Basic Card").size(24);
    let basic_card = Card::new(
        text("This is a basic card with some content. Cards can contain any widgets you want to display.")
    )
    .width(300);

    // Card with Header
    let header_title = text("Card with Header").size(24);
    let header_card = Card::new(
        text("This card has a header. Headers are great for titles and can contain any widget.")
    )
    .header(text("Card Header").size(18))
    .width(300);

    // Card with Footer
    let footer_title = text("Card with Footer").size(24);
    let footer_card = Card::new(
        text("This card has a footer with action buttons. Footers are perfect for actions or additional information.")
    )
    .header(text("Card with Actions").size(18))
    .footer(
        Row::with_children([
            Button::new("Cancel")
                .variant(ButtonVariant::Default)
                .size(ButtonSize::Small)
                .on_press(Message::ButtonPressed("Cancel".into()))
                .into(),
            Button::new("Save")
                .variant(ButtonVariant::Primary)
                .size(ButtonSize::Small)
                .on_press(Message::ButtonPressed("Save".into()))
                .into(),
        ])
        .spacing(10)
    )
    .width(300);

    // Card with All Slots
    let complete_title = text("Complete Card").size(24);
    let complete_card = Card::new(
        column([
            text("This card uses all available slots:").into(),
            text("• Image at the top").into(),
            text("• Header for the title").into(),
            text("• Body for main content").into(),
            text("• Footer for actions").into(),
        ])
        .spacing(8)
    )
    .header(
        column([
            text("Complete Example").size(18).into(),
            text("A fully featured card").size(12).into(),
        ])
        .spacing(4)
    )
    .footer(
        Row::with_children([
            Button::new("Learn More")
                .variant(ButtonVariant::Primary)
                .size(ButtonSize::Small)
                .on_press(Message::ButtonPressed("Learn More".into()))
                .into(),
        ])
    )
    .width(350);

    // Cards in a Row
    let row_title = text("Multiple Cards").size(24);
    let cards_row = Row::with_children([
        Card::new(
            column([
                text("Card 1").size(16).into(),
                text("This is the first card in a row.").into(),
            ])
            .spacing(8)
        )
        .header(text("Card 1").size(18))
        .width(250)
        .into(),
        Card::new(
            column([
                text("Card 2").size(16).into(),
                text("This is the second card in a row.").into(),
            ])
            .spacing(8)
        )
        .header(text("Card 2").size(18))
        .width(250)
        .into(),
        Card::new(
            column([
                text("Card 3").size(16).into(),
                text("This is the third card in a row.").into(),
            ])
            .spacing(8)
        )
        .header(text("Card 3").size(18))
        .width(250)
        .into(),
    ])
    .spacing(20);

    // Different Widths
    let width_title = text("Different Sizes").size(24);
    let width_examples = column([
        Card::new(text("Small card"))
            .header(text("Small").size(16))
            .width(200)
            .into(),
        Card::new(text("Medium card"))
            .header(text("Medium").size(16))
            .width(400)
            .into(),
        Card::new(text("Large card with more content to demonstrate how the card expands"))
            .header(text("Large").size(16))
            .width(600)
            .into(),
    ])
    .spacing(15);

    column![
        title,
        description,
        basic_title,
        basic_card,
        header_title,
        header_card,
        footer_title,
        footer_card,
        complete_title,
        complete_card,
        row_title,
        cards_row,
        width_title,
        width_examples,
    ]
    .spacing(20)
    .padding(20)
    .into()
}
