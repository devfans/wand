
use crate::core::State;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::component::*;
use crate::utils;

pub type SpanRef = Rc<RefCell<Span>>;
pub type SpanWeak = Weak<RefCell<Span>>;

pub trait SpanTrait {
    fn get_name(&self) -> &str;
    fn dispatch_event(&mut self, ev: &mut Event);
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d);
    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool);
}

pub type Span = Box<dyn SpanTrait>;

pub struct TextSpan {
    text: String,
}

impl SpanTrait for TextSpan {
    fn get_name(&self) -> &str {
        "Testing Text Span"
    }
    fn dispatch_event(&mut self, ev: &mut Event) {
    }
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
    }
    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool) {
        (0., 0., true)
    }

}






