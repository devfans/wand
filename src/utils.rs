use std::time::{SystemTime, UNIX_EPOCH};
use crate::prelude::{js::*, renderer::{self, Context2D, TextMetrics}};

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
    pub fn warn(s: &str);
    pub fn error(s: &str);
    pub fn debug(s: &str);
    pub fn info(s: &str);
}

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        $crate::utils::log(&format!( $( $t )* ));
    }
}
#[macro_export]
macro_rules! info {
    ( $( $t:tt )* ) => {
        $crate::utils::info(&format!( $( $t )* ));
    }
}
#[macro_export]
macro_rules! warn {
    ( $( $t:tt )* ) => {
        $crate::utils::warn(&format!( $( $t )* ));
    }
}
#[macro_export]
macro_rules! error {
    ( $( $t:tt )* ) => {
        $crate::utils::error(&format!( $( $t )* ));
    }
}
#[macro_export]
macro_rules! debug {
    ( $( $t:tt )* ) => {
        $crate::utils::debug(&format!( $( $t )* ));
    }
}

pub fn get_font_with_limit(ctx: &Context2D, text: &str, size: f64, font: &str) -> u32 {
    let mut px = 5;
    if text.trim().len() < 1 {
        return px;
    }

    let mut style: String;
    let mut res: Result<TextMetrics, JsValue>;
    for _ in 0..1000 {
        style = format!("{}px {}", px, font);
        ctx.set_font(&style);
        res = ctx.measure_text(text);
        if res.unwrap().width() >= size {
            return px;
        }
        px += 2;
    }
    px
}

#[allow(dead_code)]
#[inline]
pub fn now_ms() -> u128 {
    renderer::window().unwrap().performance().unwrap().now() as u128
    /*
    let now = web_sys::window().unwrap().performance().unwrap().now();
    let secs = now as u64 / 1000;
    let nanos = (now as u32 % 1000) * 1000000;
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    */
}

