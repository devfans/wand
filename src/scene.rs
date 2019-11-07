use std::rc::Rc;
use std::cell::RefCell;

use crate::core::{State, CanvasMeta};
use crate::traits::*;
use crate::container::{Scrollable, Container};
use crate::section::*;
use crate::content::Content;
use crate::component::Event;
use crate::span::*;
use crate::prelude::renderer::RendererContext;


pub struct Scene {
    pub path: String,
    container: Container,

    pub x: f64,
    pub y: f64,
    pub h: f64,
    pub w: f64,

    margin_x: f32,
    margin_y: f32,
    margin_min_x: f32,
    margin_max_x: f32,
    margin_min_y: f32,
    margin_max_y: f32,

    state: State
}

impl Scene {
    pub fn default(state: State) -> Self {
        Self {
            path: String::new(),
            container: Container::new(0.02, 0.02, 4., 4., 8., 8., Scrollable::None),

            x: 0.,
            y: 0.,
            h: 0.,
            w: 0.,
            margin_x: 0.2,
            margin_y: 0.2,
            margin_min_x: 10.,
            margin_max_x: 20.,
            margin_min_y: 10. ,
            margin_max_y: 20.,
            state,
        }
    }

    pub fn new(
        state: State,
        path: &str,
        margin_x: f32,
        margin_y: f32,
        margin_min_x: f32,
        margin_max_x: f32,
        margin_min_y: f32,
        margin_max_y: f32,
        container: Container,
    ) -> Self {
        Self {
            path: path.to_string(),
            container,
            x: 0.,
            y: 0., 
            h: 0.,
            w: 0.,
            margin_x,
            margin_y,
            margin_min_x,
            margin_max_x,
            margin_min_y,
            margin_max_y,
            state,
        }
    }


    /// Deprecated
    pub fn draw(&self, ctx: &RendererContext) {
        self.container.draw(ctx);
    }

    pub fn render_tick(&self, ctx: &RendererContext) {
        self.draw_outline(ctx);
        self.container.render_tick(ctx);
    }
    
    pub fn tick(&mut self) {
        self.container.tick();
    }

    pub fn on_resize(&mut self, meta: &CanvasMeta) {
        let margin_x = (meta.w as f32 * self.margin_x).max(self.margin_min_x).min(self.margin_max_x) as f64;
        let margin_y = (meta.h as f32 * self.margin_y).max(self.margin_min_y).min(self.margin_max_y) as f64;
        self.h = meta.h as f64 - 2.0 * margin_x;
        self.w = meta.w as f64 - 2.0 * margin_y;
        self.x = margin_x;
        self.y = margin_y;

        self.container.on_resize(self.x, self.y, self.w, self.h);
    }
   
    fn consume_event(&mut self, _ev: &mut Event) {
    }

    pub fn dispatch_event(&mut self, ev: &mut Event) {
        if ev.pos.in_rec(self.x, self.y, self.w, self.h) {
            self.container.dispatch_event(ev);
            self.consume_event(ev);
        }
    }


    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
    }

    pub fn register_section(&mut self, section: &SectionRef) {
        {
            let mut state = self.state.borrow_mut();
            state.register_section(section);
        }

        self.container.register(Content::Section { section: section.clone() });
    }

    pub fn add_section(&mut self, section: &SectionRef) {
        self.container.register(Content::Section { section: section.clone() });
    }

    pub fn register_span<T: 'static + SpanTrait>(&mut self, span: T) {
        let span = Rc::new(RefCell::new(Box::new(span) as Box<dyn SpanTrait>));
        {
            let mut state = self.state.borrow_mut();
            state.register_span(&span);
        }
        self.container.register(Content::Span { span });
    }
   
    pub fn add_span<T: 'static + SpanTrait>(&mut self, span: T) {
        let span = Box::new(span) as Box<dyn SpanTrait>;
        self.container.register(Content::Span { span: Rc::new(RefCell::new(span)) });
    }


}

