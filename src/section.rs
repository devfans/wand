
use wasm_bindgen::prelude::*;
use crate::container::*;
use crate::content::*;
use crate::traits::*;
use crate::utils;
use crate::component::*;
use crate::span::*;

use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::core::State;

pub type SectionRef = Rc<RefCell<Section>>;
pub type SectionWeak = Weak<RefCell<Section>>;


pub struct Section {
    pub name: String,
    container: Container,

    x: f64,
    y: f64,
    w: f64,
    h: f64,

    pub width: f32,
    pub height: f32,

    state: State
}

impl Section {
    pub fn new(state: State, name: &str, width: f32, height: f32, padding: f32) -> SectionRef {
        Rc::new(RefCell::new(Self {
            name: name.to_string(),
            container: Container::new(padding, padding, 10., 10., 20., 20., Scrollable::None),
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,
            width,
            height,
            state,
        }))
    }

    pub fn new_with_container(state: State, name: &str, width: f32, height: f32, container: Container) -> SectionRef {
        Rc::new(RefCell::new(Self {
            name: name.to_string(),
            container,
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,
            width,
            height,
            state
        }))
    }

    pub fn add_section(&mut self, section: &SectionRef) {
        {
            let mut state = self.state.borrow_mut();
            state.register_section(section);
        }

        self.container.register(Content::Section { section: section.clone() });
    }

    pub fn add_span(&mut self, span: Span) {
        let span = Rc::new(RefCell::new(span));
        {
            let mut state = self.state.borrow_mut();
            state.register_span(&span);
        }

        self.container.register(Content::Span { span: span });
    }


    pub fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool) {
        utils::log(&format!("Resizing {}", &self.name));
        self.x = left;
        self.y = top;
        self.w = self.width as f64 * (right - left);
        self.h = self.height as f64 * (bottom - top);
        self.container.on_resize(self.x, self.y, self.w, self.h);
        (self.w, self.h, true)
    }
    
    fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("white"));
        ctx.stroke_rect(self.x, self.y, self.w, self.h);
    }


    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        utils::log(format!("section {} outline {} {} {} {}", &self.name, self.x, self.y, self.w, self.h).as_str());
        self.draw_outline(ctx);
        self.container.draw(ctx);
    }

    fn consume_event(&mut self, ev: &mut Event) {
        utils::log(&format!("mouse on {} ", self.name));
    }

    pub fn dispatch_event(&mut self, ev: &mut Event) {
        if ev.pos.in_rec(self.x, self.y, self.w, self.h) {
            self.container.dispatch_event(ev);
            self.consume_event(ev);
        }
    }


}



