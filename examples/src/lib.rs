mod utils;

use wand;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("test");
}

pub fn start() {
    let app = wand::core::Application::new_with_canvas_id("canvas");
    app.draw();
}

#[wasm_bindgen]
pub struct Application {
    app: wand::core::Application,
}

#[wasm_bindgen]
impl Application {
    pub fn new() -> Self {
        let mut app = wand::Application::new_with_canvas_id("canvas");
        let state = app.get_state();
        let mut scene = wand::Scene::default(state);
        let mut section1 = app.new_section("section1", 1., 0.3, 0.01);
        let section2 = app.new_section("section2", 0.8, 0.5, 0.01);
        let section3 = app.new_section("section3", 0.5, 0.5, 0.2);
        let section4 = app.new_section("section4", 1., 0.5, 0.2);
        let section5 = app.new_section("section5", 1.,1., 0.2);
        {
            let mut sec = section1.borrow_mut();
            sec.add_section(&section3);
        }
        scene.add_section(&section1);
        scene.add_section(&section2);
        scene.add_section(&section4);
        scene.add_section(&section5);
        app.register(scene);

        Self {
            app,
        }
    }

    pub fn draw(&self) {
        self.app.draw();
    }

    pub fn on_size_change(&mut self) {
        self.app.on_resize();
    }

    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        self.app.on_mouse_move(x, y);
    }
}
