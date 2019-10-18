use std::cell::RefCell;
use std::any::Any;
use wasm_bindgen::prelude::*;

use crate::core::State;
use crate::span::SpanTrait;
use crate::component::Event;
use crate::utils;

pub struct TextSpan {
    pub name: String,
    text: String,

    x: f64,
    y: f64,
    w: f64,
    h: f64,

    pub width: f32,
    pub height: f32,

    state: State,
    font_cache: RefCell<Option<String>>, // Caching proper font for the string
}

impl TextSpan {
    pub fn new(state: State, name: &str, text: &str, width: f32, height: f32) -> Self {
        Self {
            name: name.to_string(),
            text: text.to_string(),
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,

            width,
            height,
            state,
            font_cache: RefCell::new(None),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }
}

impl SpanTrait for TextSpan {

    fn get_name(&self) -> &str {
        &self.name
    }

    fn dispath(&mut self, data: Box<dyn Any>) {
        if let Ok(text) = data.downcast::<String>() {
            self.text = text.to_string();
        }
    }

    /// Deprecated
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        self.render_tick(ctx);
    }

    fn render_tick(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let mut font = self.font_cache.borrow_mut();
        if font.is_none() {
            let size = utils::get_font_with_limit(ctx, &self.text, (self.w * 0.8).min(100.), "Arial").min(20).max(10);
            *font = Some(format!("{}px {}", size, "Arial"));
        }
        if !font.is_none() {
            ctx.set_font(font.as_ref().unwrap());
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_fill_style(&JsValue::from_str("white"));
            let _ = ctx.fill_text(&self.text, self.x + self.w/2., self.y + self.h/2.);
        }

    }

    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool) {
        self.x = left;
        self.y = top;
        self.w = self.width as f64 * (right - left);
        self.h = self.height as f64 * (bottom - top);
        // Clear font cache
        let mut font = self.font_cache.borrow_mut();
        *font = None;
        (self.w, self.h, true)
    }

}


