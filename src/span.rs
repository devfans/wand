
use crate::core::State;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub type SpanRef = Rc<RefCell<Span>>;
pub type SpanWeak = Weak<RefCell<Span>>;

pub struct Span {
    pub name: String,
}
