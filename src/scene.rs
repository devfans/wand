use crate::core::CanvasMeta;

pub fn draw_scene () {
    println!("Drawing scenine");
}

pub fn get_scene_name() -> String {
    String::from("ss")
}


pub struct Scene {
    pub path: String,
}

impl Scene {
    pub fn default() -> Self {
        Self {
            path: String::new(),
        }
    }


    pub fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
    }

    pub fn on_size_changed(&mut self, meta: &CanvasMeta) {}
}
