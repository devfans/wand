use wasm_bindgen::prelude::*;
use crate::scene::Scene;
use crate::content::Content;
use crate::container::Container;
use crate::section::Section;

pub trait DrawOutline {
    fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d);
}

#[macro_export]
macro_rules! impl_draw_outline {
    ($type: ty) => {
        impl DrawOutline for $type {
            fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d) {
                ctx.set_stroke_style(&JsValue::from_str("#07ce88"));
                ctx.stroke_rect(self.x, self.y, self.w, self.h);
            }
        }
    };
    ($type: ty, $t: ident) => {
        impl<$t: ContentItem> DrawOutline for $type {
            fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d) {
                ctx.set_stroke_style(&JsValue::from_str("white"));
                ctx.stroke_rect(self.x, self.y, self.w, self.h);
            }
        }
    }

}

impl_draw_outline!(Scene);

impl DrawOutline for Container {
    fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("white"));
        ctx.stroke_rect(self.left, self.top, self.right - self.left, self.bottom - self.top);
    }
}


pub trait ContentResize {
    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64);
}


