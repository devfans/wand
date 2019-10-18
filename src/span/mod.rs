
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::component::*;
use std::any::Any;

pub type SpanRef = Rc<RefCell<Span>>;
pub type SpanWeak = Weak<RefCell<Span>>;

pub trait SpanTrait {
    fn get_name(&self) -> &str;
    fn dispatch_event(&mut self, _ev: &mut Event) {}
    fn dispath(&mut self, _data: Box<dyn Any>) {}
    fn draw(&self, _ctx: &web_sys::CanvasRenderingContext2d) {}
    fn tick(&mut self) {}
    fn render_tick(&self, _ctx: &web_sys::CanvasRenderingContext2d) {}
    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool);
    fn get_order(&self) -> u8 { 0 }
}

// pub type Span = Box<dyn SpanTrait<Data = dyn Any>>;
pub type Span = Box<dyn SpanTrait>;

mod text_span;
mod world_span;


pub use self::{
    text_span::TextSpan,
    world_span::WorldSpan,
};




