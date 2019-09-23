use wasm_bindgen::prelude::*;
use crate::scene::Scene;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


pub trait DrawOutline {
    fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d);
}

macro_rules! impl_draw_outline {
    ($type: ty) => {
        impl DrawOutline for $type {
            fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d) {
                ctx.set_stroke_style(&JsValue::from_str("white"));
                ctx.stroke_rect(self.x - self.w/2., self.y - self.h/2., self.w, self.h);
            }
        }
    }
}

impl_draw_outline!(Scene);
