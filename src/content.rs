use crate::section::*;
use crate::component::*;
use crate::span::*;
use crate::prelude::renderer::RendererContext;

pub enum Alignment {
    Left,
    Right,
    Top,
    Bottom,
}

pub enum Content {
    Section {
        section: SectionRef,
    },
    Span {
        span: SpanRef,
    }
}

impl Content {
    pub fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool) {
        match self {
            Content::Section { ref section } => {
                let mut section = section.borrow_mut();
                section.on_resize(left, top, right, bottom)
            },
            Content::Span { ref span } => {
                let mut span = span.borrow_mut();
                let span = span.as_mut();
                span.on_resize(left, top, right, bottom);
                (0., 0., true)
            }
        }
    }

    /// Deprecated
    pub fn draw(&self, ctx: &RendererContext) {
        match self {
            Content::Section { ref section } => {
                let section = section.borrow();
                section.draw(ctx);
            },
            Content::Span { ref span } => {
                let span = span.borrow();
                span.draw(ctx);
            }
        }

    }

    pub fn render_tick(&self, ctx: &RendererContext) {
        match self {
            Content::Section { ref section } => {
                let section = section.borrow();
                section.render_tick(ctx);
            },
            Content::Span { ref span } => {
                let span = span.borrow();
                span.render_tick(ctx);
            }
        }
    }

    pub fn tick(&mut self) {
        match self {
            Content::Section { ref mut section } => {
                let mut section = section.borrow_mut();
                section.tick();
            },
            Content::Span { ref mut span } => {
                let mut span = span.borrow_mut();
                span.tick();
            }
        }
    }

    pub fn get_order_value(&self) -> u8 {
        match self {
            Content::Section { ref section } => {
                section.borrow().order
            },
            Content::Span { ref span } => {
                span.borrow().get_order()
            }
        }
    }

    pub fn dispatch_event(&mut self, ev: &mut Event) {
        match self {
            Content::Section { ref section } => {
                let mut section = section.borrow_mut();
                section.dispatch_event(ev);
            },
            Content::Span { ref span } => {
                let mut span = span.borrow_mut();
                let span = span.as_mut();
                span.dispatch_event(ev);
            }
        }
    }

}
