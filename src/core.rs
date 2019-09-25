use std::cell::{RefCell, Cell};

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::scene::Scene;
use crate::traits::*;
use crate::component::*;
use crate::section::*;
use crate::container::Container;
use crate::span::*;
use crate::utils;
use std::any::Any;

use std::rc::{Weak, Rc};

pub struct CanvasMeta {
    pub w: u32,
    pub h: u32,
}

pub struct StateProto {
    // store: HashMap::<String, Box<dyn Any>>
    sections: HashMap::<String, SectionWeak>,
    spans: HashMap::<String, SpanWeak>,
}

impl StateProto {
    pub fn new() -> State {
        Rc::new(RefCell::new(StateProto {
            sections: HashMap::new(),
            spans: HashMap::new(),
        }))
    }

    pub fn register_section(&mut self, section: &SectionRef) {
        let item = section.borrow();
        self.sections.insert(item.name.to_string(), Rc::downgrade(section));
    }


    pub fn register_span(&mut self, span: &SpanRef) {
        let item = span.borrow();
        let item = item.as_ref();
        self.spans.insert(item.get_name().to_string(), Rc::downgrade(span));
    }

    pub fn fetch_section(&self, name: &str) -> Option<SectionRef> {
        match self.sections.get(name) {
            Some(item) => item.upgrade(),
            None => None,
        }
    }

    pub fn fetch_span(&self, name: &str) -> Option<SpanRef> {
        match self.spans.get(name) {
            Some(item) => item.upgrade(),
            None => None,
        }
    }

}

pub type State = Rc<RefCell<StateProto>>;

pub struct Application {
    document: web_sys::Document,
    canvas: web_sys::HtmlCanvasElement, 

    scenes: HashMap<String, Scene>,
    path: String,
    context: web_sys::CanvasRenderingContext2d, 
    meta: CanvasMeta,

    state: State,
}

impl Application {
    pub fn new_with_canvas_id(canvas_id: &str) -> Self {
        utils::set_panic_hook();
        let document = Self::get_document();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let mut scenes = HashMap::new();
        let state = StateProto::new();
        // Fill in the default scene
        let scene = Scene::default(state.clone());
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
            state,
        };
        app.update_canvas_meta();
        app.on_resize();
        app
    }

    pub fn register(&mut self, scene: Scene) {
        // If register with custom scenes the default scene will be removed
        if self.path.is_empty() {
            self.scenes.clear();
        }
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
        // Clear first?
        self.context.clear_rect(0., 0., self.canvas.width() as f64, self.canvas.height() as f64);
        let scene = self.scenes.get(&self.path).unwrap();
        scene.draw(&self.context);
    }

    fn run(&self) {
    }

    fn update_canvas_meta(&mut self)  {
        self.meta.w = self.canvas.width();
        self.meta.h = self.canvas.height();
    }

    pub fn on_resize(&mut self) {
        self.update_canvas_meta();
        let scene = self.scenes.get_mut(&self.path).unwrap();
        scene.on_resize(&self.meta);
        /*
        for scene in self.scenes.values_mut() {
            scene.on_resize(&self.meta);
        }
        */
    }

    pub fn on_mouse_move(&mut self, x: f64, y: f64) {
        let pos = Position::new(x, y);
        let mut ev = Event { ev: EventType::MouseMove, pos, consumed: false };
        let scene = self.scenes.get_mut(&self.path).unwrap();
        scene.dispatch_event(&mut ev);
    }


    pub fn new_section(&self, name: &str, width: f32, height: f32, padding: f32) -> SectionRef {
        Section::new(self.state.clone(), name, width, height, padding)
    }

    pub fn new_section_with_container(&self, name: &str, width: f32, height: f32, container: Container) -> SectionRef {
        Section::new_with_container(self.state.clone(), name, width, height, container)
    }

    pub fn get_state(&self) -> State {
        self.state.clone()
    }

}
