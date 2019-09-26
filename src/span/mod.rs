
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::component::*;
use std::any::Any;

pub type SpanRef = Rc<RefCell<Span>>;
pub type SpanWeak = Weak<RefCell<Span>>;

pub trait SpanTrait {
    fn get_name(&self) -> &str;
    fn dispatch_event(&mut self, ev: &mut Event);
    fn dispath(&mut self, data: Box<dyn Any>);
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d);
    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool);
}

// pub type Span = Box<dyn SpanTrait<Data = dyn Any>>;
pub type Span = Box<dyn SpanTrait>;

mod text_span;


pub use self::{
    text_span::TextSpan
};




