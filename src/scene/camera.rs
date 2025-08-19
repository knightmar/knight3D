use crate::scene::object::{Object, Transform};
use nalgebra_glm::Mat4;

pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub fovy: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub(crate) fn get_view_matrix(&self) -> Mat4 {
        self.transform.get_matrix().try_inverse().unwrap() // can unwrap cause it's a square matrix
    }

    pub(crate) fn get_projection_matrix(&self) -> Mat4 {
        nalgebra_glm::perspective(f32::to_radians(self.fov), self.fovy, self.near, self.far)
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::new_empty(),
            fov: 45.0,
            fovy: 800.0 / 600.0,
            near: 0.1,
            far: 100.0,
        }
    }
}
