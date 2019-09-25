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


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

pub fn get_font_with_limit(ctx: &web_sys::CanvasRenderingContext2d, text: &str, size: f64, font: &str) -> String {
    if text.trim().len() < 1 {
        return "".to_string();
    }
    let mut px = 5;
    let mut style: String;
    let mut res: Result<web_sys::TextMetrics, JsValue>;
    for _ in 0..1000 {
        style = format!("{}px {}", px, font);
        ctx.set_font(&style);
        res = ctx.measure_text(text);
        if res.unwrap().width() >= size {
            return style;
        }
        px += 2;
    }
    "".to_string()
}
