mod renderer;

use std::cell::RefCell;
use std::any::Any;
use wasm_bindgen::prelude::*;

use crate::core::State;
use crate::span::SpanTrait;
use crate::component::Event;
use dragon::*;


pub struct WorldSpan {
    pub name: String,
    text: String,

    x: f64,
    y: f64,
    w: f64,
    h: f64,

    pub width: f32,
    pub height: f32,
    pub world: World,

    state: State,
    font_cache: RefCell<Option<String>>, // Caching proper font for the string
}

impl WorldSpan {
    pub fn new(
        state: State,
        ctx: web_sys::CanvasRenderingContext2d,
        name: &str,
        text: &str,
        width: f32,
        height: f32
    ) -> Self {
        let world = World::new();
        world.attach_default_camera();

        let renderer = renderer::RenderingSystem::new(world.state.clone(), ctx);
        world.state.register_renderer("renderer", renderer);

        Self {
            name: name.to_string(),
            text: text.to_string(),
            x: 0.,
            y: 0.,
            w: 0.,
            h: 0.,

            width,
            height,
            world,
            state,
            font_cache: RefCell::new(None),
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
    }

    fn draw_outline(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from_str("white"));
        ctx.stroke_rect(self.x, self.y, self.w, self.h);
    }

}

impl SpanTrait for WorldSpan {

    fn get_name(&self) -> &str {
        &self.name
    }

    fn dispatch_event(&mut self, _ev: &mut Event) {
    }

    fn dispath(&mut self, data: Box<dyn Any>) {
        if let Ok(text) = data.downcast::<String>() {
            self.text = text.to_string();
        }
    }

    fn render_tick(&self, _ctx: &web_sys::CanvasRenderingContext2d) {
        self.world.state.render_tick();
    }
    
    fn tick(&mut self) {
        self.world.state.tick();
    }

    /*
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        let mut font = self.font_cache.borrow_mut();
        if font.is_none() {
            let size = utils::get_font_with_limit(ctx, &self.text, (self.w * 0.8).min(100.), "Arial").min(20).max(10);
            *font = Some(format!("{}px {}", size, "Arial"));
        }
        if !font.is_none() {
            ctx.set_font(font.as_ref().unwrap());
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_fill_style(&JsValue::from_str("white"));
            let _ = ctx.fill_text(&self.text, self.x + self.w/2., self.y + self.h/2.);
        }

    }
    */

    fn on_resize(&mut self, left: f64, top: f64, right: f64, bottom: f64) -> (f64, f64, bool) {
        self.x = left;
        self.y = top;
        self.w = self.width as f64 * (right - left);
        self.h = self.height as f64 * (bottom - top);
        // Clear font cache
        let mut font = self.font_cache.borrow_mut();
        *font = None;

        // Resize world rendering system
        let mut systems = self.world.state.renderer_store.borrow_mut();
        let renderer = systems.get_mut("renderer").unwrap();
        renderer.dispatch(Box::new((self.x, self.y, self.w, self.h)));
        (0., 0., true)
    }

}

