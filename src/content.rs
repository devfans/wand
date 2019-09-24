use crate::section::*;
use crate::component::*;
use crate::span::*;

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
    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
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
