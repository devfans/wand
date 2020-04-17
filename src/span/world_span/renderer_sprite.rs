use std::rc::Rc;
use dragon::ecs::*;
use dragon::core::*;
use crate::prelude::{js::JsValue, renderer::RendererContext, renderer::Context2D };

pub struct RenderingSystem {
    state: Rc<WorldState>,
    ctx: Context2D,
    viewport: Matrix4<f32>,
}

impl RenderingSystem {
    pub fn new(state: Rc<WorldState>, ctx: &RendererContext) -> Self {
        Self {
            state,
            ctx: ctx.context_2d.clone(),
            viewport: Matrix4::identity(),
        }
    }
}

impl System for RenderingSystem {
    fn tick(&mut self) {
        let c_store = self.state.component_store.borrow();
        let sprites = c_store.get::<SpriteComponent>();
        let transforms = c_store.get::<Transform2Component>();
    }
}

