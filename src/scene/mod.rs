use crate::objects::Renderable;
use crate::scene::camera::Camera;
use crate::objects::shape::{Shape, UniformValue};
use crate::TIME;

pub mod camera;

pub struct Scene {
    pub shapes: Vec<Shape>,
    pub camera: Camera,
}

impl<'a> Scene {
    pub fn new() -> Scene {
        Scene {
            shapes: vec![],
            camera: Camera::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
    pub fn remove_shape(&mut self, i: u32) {
        self.shapes.remove(i as usize);
    }

    pub fn render(&self) {
        for shape in &self.shapes {
            shape.set_uniform(
                "model".to_string(),
                UniformValue::Matrix4fv(shape.transform.get_matrix()),
            );
            shape.set_uniform(
                "view".to_string(),
                UniformValue::Matrix4fv(self.camera.get_view_matrix()),
            );
            shape.set_uniform(
                "projection".to_string(),
                UniformValue::Matrix4fv(self.camera.get_projection_matrix()),
            );
            unsafe {
                shape.set_uniform("time".to_string(), UniformValue::Float(TIME as f32));
            }
            shape.render();
        }
    }
}
