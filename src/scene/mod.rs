use crate::scene::camera::Camera;
use crate::scene::object::Object;
use crate::shape::Shape;

pub mod camera;
pub mod object;

pub struct Scene {
    pub shapes: Vec<Shape>,
    pub camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            shapes: vec![],
            camera: Camera::new(),
        }
    }
    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
}
