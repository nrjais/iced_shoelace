use iced_widget::{Component, component, mouse_area};

use crate::{Element, components::ElementFn, theme::Theme};

pub struct Hovered<'a, Message: 'a + Clone> {
    builder: ElementFn<'a, bool, Message>,
}

impl<'a, Message: 'a + Clone> Hovered<'a, Message> {
    pub fn new(builder: impl Fn(bool) -> Element<'a, Message> + 'a) -> Self {
        Self {
            builder: Box::new(builder),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Event<Message: Clone> {
    Hovered,
    Exited,
    Message(Message),
}

#[derive(Debug, Clone, Default)]
pub struct State {
    hovered: bool,
}

impl<'a, Message: 'a + Clone> Component<'a, Message, Theme> for Hovered<'a, Message> {
    type State = State;
    type Event = Event<Message>;

    fn update(&mut self, state: &mut Self::State, event: Self::Event) -> Option<Message> {
        match event {
            Event::Hovered => {
                state.hovered = true;
            }
            Event::Exited => {
                state.hovered = false;
            }
            Event::Message(message) => return Some(message),
        }
        None
    }

    fn view(&self, state: &Self::State) -> Element<'a, Self::Event> {
        let content = (self.builder)(state.hovered).map(Event::Message);

        mouse_area(content)
            .interaction(iced::mouse::Interaction::Pointer)
            .on_enter(Event::Hovered)
            .on_exit(Event::Exited)
            .into()
    }
}

impl<'a, Message: 'a + Clone> From<Hovered<'a, Message>> for Element<'a, Message> {
    fn from(hovered: Hovered<'a, Message>) -> Self {
        component(hovered)
    }
}

pub fn hovered<'a, Message: 'a + Clone, F, I>(f: F) -> Element<'a, Message>
where
    Message: Clone + 'a,
    F: Fn(bool) -> I + 'a,
    I: Into<Element<'a, Message>> + 'a,
{
    Hovered::new(move |hovered| f(hovered).into()).into()
}
