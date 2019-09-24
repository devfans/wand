
pub struct Position(f64, f64);

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    pub fn in_rec(&self, x: f64, y: f64, w: f64, h: f64) -> bool {
        self.0 > x && self.0 < x + w && self.1 > y && self.1 < y + h
    }

    pub fn in_area(&self, left: f64, top: f64, right: f64, bottom: f64) -> bool {
        self.0 > left && self.0 < right && self.1 > top && self.1 < bottom
    }
}

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum EventType {
    MouseMove,
}

pub struct Event {
    pub pos: Position,
    pub ev: EventType,
    pub consumed: bool,
}

