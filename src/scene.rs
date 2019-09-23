use crate::core::CanvasMeta;
use crate::utils::DrawOutline;

pub fn draw_scene () {
    println!("Drawing scenine");
}

pub fn get_scene_name() -> String {
    String::from("ss")
}


pub struct Scene {
    pub path: String,

    pub x: f64,
    pub y: f64,
    pub h: f64,
    pub w: f64,
}

impl Scene {
    pub fn default() -> Self {
        Self {
            path: String::new(),
            x: 0.,
            y: 0.,
            h: 0.,
            w: 0.,
        }
    }


    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        self.draw_outline(ctx);
    }

    pub fn on_size_changed(&mut self, meta: &CanvasMeta) {
        /*
        margin_x: f64,
        margin_y: f64,
        min_margin_x: f64,
        max_margin_x: f64,
        min_margin_y: f64,
        max_margin_y: f64,
        */

        self.x = meta.w as f64 / 2.;
        self.y = meta.h as f64 /2.;
        let margin_x = (meta.w as f64 * 0.3).max(20.).min(50.);
        let margin_y = (meta.h as f64 * 0.3).max(20.).min(50.);
        self.h = meta.h as f64 - 2.0 * margin_x;
        self.w = meta.w as f64 - 2.0 * margin_y;
    }
}
