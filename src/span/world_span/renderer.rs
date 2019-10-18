use std::rc::Rc;
use dragon::ecs::*;
use dragon::core::*;
use std::any::Any;
use std::cmp::PartialOrd;
use wasm_bindgen::prelude::*;

pub struct RenderingSystem {
    state: Rc<WorldState>,
    ctx: web_sys::CanvasRenderingContext2d,
    viewport: Matrix4<f32>,
}

impl RenderingSystem {
    pub fn new(state: Rc<WorldState>, ctx: web_sys::CanvasRenderingContext2d) -> Self {
        Self {
            state,
            ctx,
            viewport: Matrix4::identity(),
        }
    }
}

impl System for RenderingSystem {
    fn tick(&mut self) {

        let c_store = self.state.component_store.borrow();
        let active_camera = self.state.active_camera.get();
        let cameras = c_store.get::<CameraComponent>();
        let camera = cameras.get(&active_camera).unwrap();
        let meshes = c_store.get::<MeshComponent>();
        let transforms = c_store.get::<TransformComponent>();
        macro_rules! project {
            ($point: expr) => {
                self.viewport.transform_point(&camera.project_point($point))
            };
            ($point: expr, $model: expr) => {
                self.viewport.transform_point(&camera.project_point(&$model.transform_point($point)))
            }

        }
       
        let mut polygons = Vec::new();
        for (mesh, transform) in meshes.iter().filter(|entity| transforms.contains_key(entity.0)).map(|(entity, mesh)| {
            (mesh, transforms.get(entity).unwrap())
        }) {
            let model = transform.matrix();
            match mesh.cook() {
                MeshRecipe::Basic { data } => {
                    let mut cutter = data.breaks.iter();
                    let count = data.vertices.len() - 1;
                    let mut cut_at = cutter.next().unwrap_or(&count);
                    // Simplified rendering to canvas 2d context 
                    self.ctx.set_stroke_style(&JsValue::from_str("white"));
                    self.ctx.begin_path();
                    let mut first = true;
                    for (index, vertex) in data.vertices.iter().enumerate() {
                        let point = project!(vertex, &model);
                        if first {
                            self.ctx.move_to(point.x as f64, point.y as f64);
                            first = false;
                        } else {
                            self.ctx.line_to(point.x as f64, point.y as f64);
                        }

                        if index == *cut_at {
                            cut_at = cutter.next().unwrap_or(&count);
                            first = true;
                        }
                    }
                    self.ctx.stroke();
                },
                MeshRecipe::Simple { data } => {
                    for i in data.polygons.iter() {
                        let v1 = project!(&data.vertices[i.0], &model);
                        let v2 = project!(&data.vertices[i.1], &model);
                        let v3 = project!(&data.vertices[i.2], &model);
                        polygons.push((
                            ((v1.coords + v2.coords + v3.coords)/3.).z,
                            v1, v2, v3,
                            i.3.clone()
                        ));

                    }
                }
            }

            // Render polygons
            polygons.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
            for p in polygons.iter().rev() {
                self.ctx.begin_path();
                self.ctx.set_fill_style(&JsValue::from_str(&p.4));
                self.ctx.set_stroke_style(&JsValue::from_str(&p.4));
                self.ctx.move_to(p.1.x as f64, p.1.y as f64);
                self.ctx.line_to(p.2.x as f64, p.2.y as f64);
                self.ctx.line_to(p.3.x as f64, p.3.y as f64);
                self.ctx.line_to(p.1.x as f64, p.1.y as f64);
                self.ctx.stroke();
                self.ctx.fill();
            }

            /*
            let mut lines = Vec::new();
            let mut line = Vec::new();
            for (index, vertex) in mesh.vertices.iter().enumerate() {
                line.push(self.viewport.transform_point(&camera.project_point(&model.transform_point(vertex))));
                if index == *cut_at {
                    lines.push(line.clone());
                    line.clear();
                    cut_at = cutter.next().unwrap_or(&count);
                }
            }
            self.ctx.set_stroke_style(&JsValue::from_str("white"));
            for line in lines {
                self.ctx.begin_path();
                let mut first = true;
                for vertex in line {
                    log!("{}", vertex);
                    if first {
                        self.ctx.move_to(vertex.x as f64, vertex.y as f64);
                        first = false;
                    } else {
                        self.ctx.line_to(vertex.x as f64, vertex.y as f64);
                    }
                }
                self.ctx.stroke();
            }
            */
        }

    }

    fn dispatch(&mut self, data: Box<dyn Any>) {
        if let Ok(vp) = data.downcast::<(f64, f64, f64, f64)>() {
            self.viewport = Matrix4::new_translation(&Vector3::new(vp.0 as f32, vp.1 as f32, 0.))
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
