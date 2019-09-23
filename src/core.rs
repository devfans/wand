use std::cell::Cell;
use std::rc::Rc;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::scene::Scene;

pub struct CanvasMeta {
    pub w: u32,
    pub h: u32,
}

pub struct Application {
    document: web_sys::Document,
    canvas: web_sys::HtmlCanvasElement, 

    scenes: HashMap<String, Scene>,
    path: String,
    context: web_sys::CanvasRenderingContext2d, 
    meta: CanvasMeta,
}

impl Application {
    pub fn new_with_canvas_id(canvas_id: &str) -> Self {
        let document = Self::get_document();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let mut scenes = HashMap::new();
        // Fill in the default scene
        let scene = Scene::default();
        let path = scene.path.clone();
        scenes.insert(scene.path.clone(), scene);
        let context = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let meta = CanvasMeta { w: canvas.width(), h: canvas.height() };
        let mut app = Self {
            document,
            canvas,
            scenes,
            path,
            context,
            meta,
        };
        app.update_canvas_meta();
        app.on_size_change();
        app
    }

    pub fn register(&mut self, scene: Scene) {
        let path = scene.path.clone();
        if self.scenes.get(&path).is_none() {
            self.scenes.insert(scene.path.clone(), scene);
            self.path = path;
        }
    }

    fn get_document() -> web_sys::Document {
        web_sys::window().unwrap().document().unwrap()
    }

    fn get_window() -> web_sys::Window {
        web_sys::window().expect("No global window exists")
    }

    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        Self::get_window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("Failed to register `requestAnimationFrame`");
    }

    pub fn draw(&self) {
        let scene = self.scenes.get(&self.path).unwrap();
        scene.draw(&self.context);
    }

    fn run(&self) {
    }

    fn update_canvas_meta(&mut self)  {
        self.meta.w = self.canvas.width();
        self.meta.h = self.canvas.height();
    }

    pub fn on_size_change(&mut self) {
        self.update_canvas_meta();
        for scene in self.scenes.values_mut() {
            scene.on_size_changed(&self.meta);
        }
    }
}
