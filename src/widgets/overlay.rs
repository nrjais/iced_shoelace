//! Overlay widget for displaying content above other elements.
//!
//! # Example
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } }
//! # pub type State = ();
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! use iced::widget::{container, button, text};
//! use overlay::Overlay;
//!
//! enum Message {
//!     ToggleOverlay,
//! }
//!
//! fn view(show_overlay: bool) -> Element<'_, Message> {
//!     if show_overlay {
//!         Overlay::new(
//!             button("Click me").on_press(Message::ToggleOverlay),
//!             container(text("Overlay content"))
//!                 .padding(20)
//!                 .style(container::rounded_box),
//!         ).into()
//!     } else {
//!         button("Show Overlay").on_press(Message::ToggleOverlay).into()
//!     }
//! }
//! ```

use iced::widget::container;
use iced_core::layout::{self, Layout};
use iced_core::mouse;
use iced_core::overlay;
use iced_core::renderer;
use iced_core::text;
use iced_core::widget::{self, Widget};
use iced_core::{Clipboard, Color, Element, Event, Length, Rectangle, Shell, Size, Vector};

/// An overlay widget that displays content on top of a backdrop.
///
/// The overlay widget is useful for creating modals, dialogs, and other
/// elements that need to appear above the rest of the UI.
pub struct Overlay<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
{
    underlay: Element<'a, Message, Theme, Renderer>,
    content: Element<'a, Message, Theme, Renderer>,
    backdrop_color: Color,
    on_backdrop_click: Option<Message>,
}

impl<'a, Message, Theme, Renderer> Overlay<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
    Message: Clone,
{
    /// Creates a new [`Overlay`].
    ///
    /// The `underlay` is the content that appears behind the overlay.
    /// The `content` is the content that appears in the overlay.
    pub fn new(
        underlay: impl Into<Element<'a, Message, Theme, Renderer>>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            underlay: underlay.into(),
            content: content.into(),
            backdrop_color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            on_backdrop_click: None,
        }
    }

    /// Sets the color of the backdrop.
    pub fn backdrop_color(mut self, color: Color) -> Self {
        self.backdrop_color = color;
        self
    }

    /// Sets the message to emit when the backdrop is clicked.
    pub fn on_backdrop_click(mut self, message: Message) -> Self {
        self.on_backdrop_click = Some(message);
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Overlay<'_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: text::Renderer,
    Message: Clone,
{
    fn children(&self) -> Vec<widget::Tree> {
        vec![
            widget::Tree::new(&self.underlay),
            widget::Tree::new(&self.content),
        ]
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&[self.underlay.as_widget(), self.content.as_widget()]);
    }

    fn size(&self) -> Size<Length> {
        self.underlay.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.underlay.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.underlay
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut widget::Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.underlay.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.underlay.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            inherited_style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let mut children = tree.children.iter_mut();

        let underlay_overlay = self.underlay.as_widget_mut().overlay(
            children.next().unwrap(),
            layout,
            renderer,
            viewport,
            translation,
        );

        let overlay = overlay::Element::new(Box::new(OverlayInstance {
            content: &mut self.content,
            tree: children.next().unwrap(),
            backdrop_color: self.backdrop_color,
            on_backdrop_click: self.on_backdrop_click.clone(),
        }));

        if let Some(underlay) = underlay_overlay {
            Some(overlay::Group::with_children(vec![underlay, overlay]).overlay())
        } else {
            Some(overlay)
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Overlay<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + 'a,
    Renderer: text::Renderer + 'a,
{
    fn from(
        overlay: Overlay<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(overlay)
    }
}

struct OverlayInstance<'a, 'b, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
{
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut widget::Tree,
    backdrop_color: Color,
    on_backdrop_click: Option<Message>,
}

impl<Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for OverlayInstance<'_, '_, Message, Theme, Renderer>
where
    Renderer: text::Renderer,
    Message: Clone,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds);

        let content_layout = self
            .content
            .as_widget_mut()
            .layout(self.tree, renderer, &limits);

        let content_size = content_layout.size();

        // Center the content
        let x = (bounds.width - content_size.width) / 2.0;
        let y = (bounds.height - content_size.height) / 2.0;

        layout::Node::with_children(bounds, vec![content_layout.translate(Vector::new(x, y))])
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let content_layout = layout.children().next().unwrap();

        // Handle backdrop clicks
        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event
            && let Some(cursor_position) = cursor.position()
                && !content_layout.bounds().contains(cursor_position)
                    && let Some(ref message) = self.on_backdrop_click {
                        shell.publish(message.clone());
                    }

        self.content.as_widget_mut().update(
            self.tree,
            event,
            content_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        inherited_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        // Draw backdrop
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                ..renderer::Quad::default()
            },
            self.backdrop_color,
        );

        // Draw content
        let content_layout = layout.children().next().unwrap();
        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            inherited_style,
            content_layout,
            cursor,
            &layout.bounds(),
        );
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let content_layout = layout.children().next().unwrap();

        self.content.as_widget().mouse_interaction(
            self.tree,
            content_layout,
            cursor,
            &layout.bounds(),
            renderer,
        )
    }
}
