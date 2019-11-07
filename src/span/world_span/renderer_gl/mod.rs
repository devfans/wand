use std::rc::Rc;
use dragon::ecs::*;
use dragon::core::*;
use std::any::Any;
use std::cmp::PartialOrd;
use std::f64::consts::PI;
use crate::prelude::{js::JsValue, renderer::RendererContext, renderer::ContextGL };

pub struct RenderingSystem {
    state: Rc<WorldState>,
    ctx: ContextGL,
    viewport: Matrix4<f32>,
}

impl RenderingSystem {
    pub fn new(state: Rc<WorldState>, ctx: &RendererContext) -> Self {
        Self {
            state,
            ctx: ctx.context_gl.clone(),
            viewport: Matrix4::identity(),
        }
    }
}

impl System for RenderingSystem {
    fn tick(&mut self) {
    }

    fn dispatch(&mut self, data: Box<dyn Any>) {
        if let Ok(vp) = data.downcast::<(f64, f64, f64, f64)>() {
            let mut flip_y = Matrix4::identity();
            flip_y.row_mut(1)[1] = -1.;

            self.viewport = Matrix4::new_translation(&Vector3::new(vp.0 as f32, (vp.1 + vp.3) as f32, 0.))
                * flip_y
                * Matrix4::new_nonuniform_scaling(&Vector3::new(vp.2 as f32 /2., vp.3 as f32 /2., 1.))
                * Matrix4::new_translation(&Vector3::new(1., 1., 0.));
            log!("New viewport set {}", &self.viewport);

            // Resize the camera projection
            let c_store = self.state.component_store.borrow();
            let mut cameras = c_store.get_mut::<CameraComponent>();
            let active_camera = self.state.active_camera.get();
            match cameras.get_mut(&active_camera).unwrap() {
                Camera::Perspective { ref mut projection } => {
                    projection.set_aspect((vp.2 / vp.3) as f32);
                },
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport() {
        let x = 100f32;
        let y = 120f32;
        let w = 40f32;
        let h = 20f32;
        let mut flip_y = Matrix4::identity();
        flip_y.row_mut(1)[1] = -1.;

        let viewport = Matrix4::new_translation(&Vector3::new(x, y + h, 0.))
            * flip_y
            * Matrix4::new_nonuniform_scaling(&Vector3::new(w /2., h /2., 1.))
            * Matrix4::new_translation(&Vector3::new(1., 1., 0.));
        println!("Viewport testing {}", viewport.transform_point(&Point3::new(1., -1., 1.)));
        assert_eq!(viewport.transform_point(&Point3::new(1., -1., 1.)), Point3::new(140., 140., 1.));
        assert_eq!(viewport.transform_point(&Point3::new(-1., 1., 1.)), Point3::new(100., 120., 1.));
        assert_eq!(viewport.transform_point(&Point3::new(-1., -1., 1.)), Point3::new(100., 140., 1.));
        assert_eq!(viewport.transform_point(&Point3::new(1., 1., 1.)), Point3::new(140., 120., 1.));
    }
}

 
