use std::cell::Cell;
use std::rc::Rc;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::scene::Scene;

pub struct CanvasMeta {}

pub struct Application {
    document: web_sys::Document,
    canvas: web_sys::HtmlCanvasElement, 

    scenes: HashMap<String, Rc<RefCell<Scene>>>,
    path: String,
    scene: Rc<Scene>,
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
        let scene = Rc::new(Scene::default());
        let path = scene.path.clone();
        scenes.insert(scene.path.clone(), scene.clone());
        let context = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        let meta = Self::get_canvas_meta(&canvas);
        Self {
            document,
            canvas,
            scenes,
            path,
            scene,
            context,
            meta,
        }
    }

    pub fn register(&mut self, scene: Scene) {
        let scene = Rc::new(scene);
        if self.scenes.get(&scene.path).is_none() {
            self.scenes.insert(scene.path.clone(), scene.clone());
            self.path = scene.path.clone();
            self.scene = scene;
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

    fn draw(&self) {
        self.scene.draw(&self.context);
    }

    fn run(&self) {
    }

    fn get_canvas_meta(canvas: &web_sys::HtmlCanvasElement) -> CanvasMeta {
        CanvasMeta {}
    }

    fn on_size_change(&mut self) {
        self.meta = Self::get_canvas_meta(&self.canvas);
        for scene in self.scenes.values_mut() {
            scene.on_size_changed(&self.meta);
        }
    }
}
