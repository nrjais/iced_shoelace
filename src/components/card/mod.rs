use crate::{
    Element,
    theme::{container::ContainerStyleClass, sizes::SPACING},
};
use iced::{
    Length,
    widget::{column, container},
};

/// A Shoelace-style card component for iced
///
/// This component implements the features from Shoelace's card component:
/// - Header slot for titles and headings
/// - Default slot for main content
/// - Footer slot for actions
/// - Image slot for hero images
/// - Configurable padding and borders
/// - Shadow styling
pub struct Card<'a, Message> {
    header: Option<Element<'a, Message>>,
    content: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    image: Option<Element<'a, Message>>,
    width: Length,
    height: Length,
    padding: f32,
}

impl<'a, Message> Card<'a, Message>
where
    Message: Clone + 'a,
{
    /// Creates a new card with the given content
    pub fn new(content: impl Into<Element<'a, Message>>) -> Self {
        Self {
            header: None,
            content: content.into(),
            footer: None,
            image: None,
            width: Length::Shrink,
            height: Length::Shrink,
            padding: SPACING.large,
        }
    }

    /// Sets the header element (displayed at the top of the card)
    pub fn header(mut self, header: impl Into<Element<'a, Message>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Sets the footer element (displayed at the bottom of the card)
    pub fn footer(mut self, footer: impl Into<Element<'a, Message>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Sets the image element (displayed at the very top, before the header)
    pub fn image(mut self, image: impl Into<Element<'a, Message>>) -> Self {
        self.image = Some(image.into());
        self
    }

    /// Sets the width of the card
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the card
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the padding of the card content (default is SPACING.large)
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }
}

impl<'a, Message: 'a> From<Card<'a, Message>> for Element<'a, Message>
where
    Message: Clone,
{
    fn from(card: Card<'a, Message>) -> Self {
        let mut card_column = column([]).spacing(0).width(card.width).height(card.height);

        // Add image if present (no padding, full width)
        if let Some(image) = card.image {
            // Wrap image in a container with top border radius matching the card
            let image_container = container(image)
                .width(Length::Fill)
                .class(ContainerStyleClass::CardImage);

            card_column = card_column.push(image_container);
        }

        // Add header if present (with padding)
        // Shoelace uses --sl-spacing-large for all card sections
        if let Some(header) = card.header {
            let header_container = container(header)
                .width(Length::Fill)
                .padding(card.padding)
                .class(ContainerStyleClass::CardHeader);

            card_column = card_column.push(header_container);
        }

        // Add main content (with padding)
        // Shoelace applies consistent padding to body content
        let content_container = container(card.content)
            .width(Length::Fill)
            .padding(card.padding)
            .class(ContainerStyleClass::CardContent);

        card_column = card_column.push(content_container);

        // Add footer if present (with padding)
        if let Some(footer) = card.footer {
            let footer_container = container(footer)
                .width(Length::Fill)
                .padding(card.padding)
                .class(ContainerStyleClass::CardFooter);

            card_column = card_column.push(footer_container);
        }

        // Wrap everything in the main card container with border and shadow
        container(card_column)
            .width(card.width)
            .height(card.height)
            .class(ContainerStyleClass::Card)
            .into()
    }
}
