use crate::scene::camera::Camera;
use crate::scene::object::Object;
use crate::scene::shape::{Shape, UniformValue};
use crate::TIME;

pub mod camera;
pub mod object;
pub mod shape;

pub struct Scene<'a> {
    pub shapes: Vec<Shape<'a>>,
    pub camera: Camera,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Scene {
            shapes: vec![],
            camera: Camera::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Shape<'a>) {
        self.shapes.push(shape);
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
