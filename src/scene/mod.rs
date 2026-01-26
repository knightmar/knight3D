use crate::scene::camera::Camera;
use crate::scene::object::Object;
use crate::shape::{Shape, UniformValue};
use crate::TIME;

pub mod camera;
pub mod object;

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
                "model",
                UniformValue::Matrix4fv(shape.transform.get_matrix()),
            );
            shape.set_uniform(
                "view",
                UniformValue::Matrix4fv(self.camera.get_view_matrix()),
            );
            shape.set_uniform(
                "projection",
                UniformValue::Matrix4fv(self.camera.get_projection_matrix()),
            );
            unsafe {
                shape.set_uniform("time", UniformValue::Float(TIME as f32));
            }
            shape.render();
        }
    }
}
