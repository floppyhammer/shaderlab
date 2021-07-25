use iced_native::{Clipboard, Hasher};
use iced_wgpu::{Defaults, Primitive, Renderer};
use iced_winit::{
    event, layout, mouse, overlay, Element, Event, Layout, Length, Point, Rectangle, Widget,
};
use std::cell::Cell;

pub struct Extractor<'a, Message> {
    inner: Element<'a, Message, Renderer>,
    bounds: &'a Cell<Rectangle>,
}

impl<'a, Message: Clone> Extractor<'a, Message> {
    pub fn new(
        bounds: &'a Cell<Rectangle>,
        inner: impl Into<Element<'a, Message, Renderer>>,
    ) -> Self {
        let inner = inner.into();
        Self { bounds, inner }
    }
}

impl<'a, Message> Widget<Message, Renderer> for Extractor<'a, Message> {
    fn width(&self) -> Length {
        self.inner.width()
    }

    fn height(&self) -> Length {
        self.inner.height()
    }

    fn layout(&self, renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        self.inner.layout(renderer, limits)
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> event::Status {
        self.inner.on_event(
            event,
            layout,
            cursor_position,
            renderer,
            clipboard,
            messages,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> (Primitive, mouse::Interaction) {
        self.bounds.set(layout.bounds());
        self.inner
            .draw(renderer, defaults, layout, cursor_position, viewport)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.inner.hash_layout(state);
    }

    fn overlay(&mut self, layout: Layout<'_>) -> Option<overlay::Element<'_, Message, Renderer>> {
        self.inner.overlay(layout)
    }
}

impl<'a, Message: 'a> From<Extractor<'a, Message>> for Element<'a, Message, Renderer> {
    fn from(widget: Extractor<'a, Message>) -> Element<'a, Message, Renderer> {
        Element::new(widget)
    }
}
