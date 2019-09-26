use crate::traits::*;
use crate::content::Content;
use crate::component::*;

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum Scrollable {
    X = 0,
    Y = 1,
    None = 2,
}

pub struct Container {
    scroll: Scrollable,

    x: f64,
    y: f64,

    pub left: f64, 
    pub right: f64,
    pub top: f64,
    pub bottom: f64,

    inventory: Vec<Content>,
    
    padding_x: f32,
    padding_y: f32,
    padding_min_x: f32,
    padding_max_x: f32,
    padding_min_y: f32,
    padding_max_y: f32,

}

impl Container {
    pub fn new(
        padding_x: f32,
        padding_y: f32,
        padding_min_x: f32,
        padding_max_x: f32,
        padding_min_y: f32,
        padding_max_y: f32,
        scroll: Scrollable
    ) -> Self {
        Self {
            scroll,

            x: 0.,
            y: 0.,

            left: 0.,
            right: 0.,
            top: 0.,
            bottom: 0.,
            
            inventory: Vec::new(),

            padding_x,
            padding_y,
            padding_min_x,
            padding_max_x,
            padding_min_y,
            padding_max_y,

        }
    }

    pub fn on_resize(&mut self, x: f64, y: f64, w: f64, h: f64) {
        let padding_x = (w as f32 * self.padding_x).max(self.padding_min_x).min(self.padding_max_x) as f64;
        let padding_y = (w as f32 * self.padding_y).max(self.padding_min_y).min(self.padding_max_y) as f64;
        self.left = x + padding_x;
        self.top = y + padding_y;
        self.right = x + w - padding_x;
        self.bottom = y + h - padding_y;
        self.x = self.left;
        self.y = self.top;

        self.resize();
    }


    fn resize(&mut self) {
        for i in 0..self.inventory.len(){
            let item = self.inventory.get_mut(i).unwrap();
            let w;
            let h;
            loop {
                let (width, height, fit) = (*item).on_resize(self.x, self.y, self.right, self.bottom);
                log!("resize content item: {} {} {} {}, {} {} {}", self.x, self.y, self.right, self.bottom, width, height, fit);
                if fit {
                    w = width;
                    h = height;
                    break;
                }
            }
            self.update_cursor(w, h);
        }
    }

    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        // utils::log(format!("container outline {} {} {} {}", self.left, self.right, self.top, self.bottom).as_str());

        self.draw_outline(ctx);
        for item in self.inventory.iter() {
            item.draw(ctx);
        }
    }

    pub fn update_cursor(&mut self, w: f64, h: f64) {
        // utils::log(&format!("updating cursor {} {}", w, h));
        // utils::log(&format!(" cursor {} {} {} {}", self.x, self.y, self.right, self.bottom));

        match self.scroll {
            Scrollable::X => {
                self.x += w;
                if self.y >= self.bottom {
                    self.y = self.top;
                }
            },
            Scrollable::Y => {
                self.y += h;
                if self.x >= self.right {
                    self.x = self.left;
                }
            },
            Scrollable::None => {
                self.x += w;
                if self.x >= self.right {
                    self.x = self.left;
                    self.y += h;
                }

            }
        }
        // utils::log(&format!("updated cursor {} {}", self.x, self.y));
    }

    pub fn register(&mut self, item: Content) {
        self.inventory.push(item);
    }

    fn consume_event(&mut self, ev: &mut Event) {
    }

    pub fn dispatch_event(&mut self, ev: &mut Event) {
        if ev.pos.in_area(self.left, self.top, self.right, self.bottom) {
            for item in self.inventory.iter_mut() {
                item.dispatch_event(ev);
            }
            self.consume_event(ev);
        }
    }

}


