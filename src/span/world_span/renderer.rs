use std::rc::Rc;
use dragon::ecs::*;
use dragon::core::*;
use std::any::Any;
use std::cmp::PartialOrd;
use std::f64::consts::PI;
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
        let shape_store = self.state.shape_store.borrow();
        let active_camera = self.state.active_camera.get();
        let cameras = c_store.get::<CameraComponent>();
        let camera = cameras.get(&active_camera).unwrap();
        let meshes = c_store.get::<MeshComponent>();
        let transforms = c_store.get::<TransformComponent>();
        let camera_transform = transforms.get(&active_camera).unwrap()
            .isometry().inverse().to_homogeneous();
        let vp = self.viewport * camera.as_matrix() * camera_transform;

        macro_rules! project_size {
            ($radius: expr, $model: expr) => {
                camera.transform_size($radius, camera_transform.transform_point($model).z.abs())
            }
        }
       
        let mut polygons = Vec::new();
        for (mesh, transform) in meshes.iter().filter(|entity| transforms.contains_key(entity.0)).map(|(entity, mesh)| {
            (mesh, transforms.get(entity).unwrap())
        }) {
            let model = transform.matrix();
            let mvp = vp * model;
            let translation = transform.position();
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
                        let point = mvp.transform_point(vertex);
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
                        let v1 = mvp.transform_point(&data.vertices[i.0]);
                        let v2 = mvp.transform_point(&data.vertices[i.1]);
                        let v3 = mvp.transform_point(&data.vertices[i.2]);
                        polygons.push((
                            ((v1.coords + v2.coords + v3.coords)/3.).z,
                            v1, v2, v3,
                            i.3.clone()
                        ));

                    }
                }
                MeshRecipe::Complex { data } => {
                    for brush in data.brushes.iter() {
                        match brush {
                            Brush::Lines { stroke, fill, vertices, action } => {
                                if let Some(stroke) = stroke {
                                    self.ctx.set_stroke_style(&JsValue::from_str(&stroke));
                                }
                                if let Some(fill) = fill {
                                    self.ctx.set_fill_style(&JsValue::from_str(&fill));
                                }
                                self.ctx.begin_path();
                                let mut first = true;
                                for vertex in vertices.iter() {
                                    let point = mvp.transform_point(vertex);
                                    if first {
                                        self.ctx.move_to(point.x as f64, point.y as f64);
                                        first = false;
                                    } else {
                                        self.ctx.line_to(point.x as f64, point.y as f64);
                                    }
                                }
                                if action & 0x01 > 0 {
                                    self.ctx.fill();
                                }
                                if action & 0x02 > 0 {
                                    self.ctx.stroke();
                                }
                            },
                            Brush::Sphere { stroke, fill, center, radius, action } => {
                                if let Some(stroke) = stroke {
                                    self.ctx.set_stroke_style(&JsValue::from_str(&stroke));
                                }
                                if let Some(fill) = fill {
                                    self.ctx.set_fill_style(&JsValue::from_str(&fill));
                                }
                                self.ctx.begin_path();
                                let point = mvp.transform_point(center);
                                let radius = project_size!(*radius, &translation);
                                let _ = self.ctx.arc(point.x as f64, point.y as f64, radius as f64, 0., PI*2.);
                                if action & 0x01 > 0 {
                                    self.ctx.fill();
                                }
                                if action & 0x02 > 0 {
                                    self.ctx.stroke();
                                }
                            }
                            _ => {},
                        }
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

            // Render shapes
            for shape in shape_store.iter() {
                self.ctx.set_stroke_style(&JsValue::from_str("white"));
                match shape {
                    Shape::Line { begin, end } => {
                        let begin = vp.transform_point(begin);
                        let end = vp.transform_point(end);
                        self.ctx.begin_path();
                        self.ctx.move_to(begin.x as f64, begin.y as f64);
                        self.ctx.line_to(end.x as f64, end.y as f64);
                        self.ctx.stroke();
                    },
                    Shape::Circle { center, radius } => {
                        let center = vp.transform_point(&Point3::from(*center.translation()));
                        self.ctx.begin_path();
                        let _ = self.ctx.ellipse(
                            center.x as f64,
                            center.y as f64,
                            *radius as f64,
                            *radius as f64,
                            0., 0., PI * 2.);
                        self.ctx.stroke();
                    },
                    _ => {},
                }
            }

            let widgets = c_store.get::<WidgetComponent>();
            self.ctx.set_text_align("center");
            self.ctx.set_text_baseline("middle");
            self.ctx.set_fill_style(&JsValue::from_str("grey"));
            self.ctx.set_stroke_style(&JsValue::from_str("darkgreen"));
            for (entity, widget) in widgets.iter() {
                let translate = transforms.get(entity).unwrap().position();
                let position = vp.transform_point(&translate);
                let scale = project_size!(1f32, &translate);
                match widget {
                    Widget::Text { ref translation, ref text } => {
                        let center = position + translation * scale;
                        let _ = self.ctx.fill_text(text, center.x as f64, center.y as f64);
                    },
                    Widget::FramedText { ref translation, ref text, width, height } => {
                        let w = width * scale;
                        let h = height * scale;
                        let center = position + translation * project_size!(1f32 , &translate);
                        let _ = self.ctx.fill_text(text, center.x as f64, center.y as f64);
                        self.ctx.rect((center.x - w / 2.) as f64, (center.y - h /2.) as f64, w as f64, h as f64);
                        self.ctx.stroke();
                    }
                }
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
