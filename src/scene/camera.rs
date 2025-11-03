use crate::scene::object::{Object, Transform};
use crate::ui::Inspectable;
use imgui::Ui;
use nalgebra_glm::Mat4;

#[derive(Clone, Copy)]
pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
}

impl Camera {
    pub(crate) fn get_view_matrix(&self) -> Mat4 {
        self.transform.get_matrix().try_inverse().unwrap() // can unwrap cause it's a square matrix
    }

    pub(crate) fn get_projection_matrix(&self) -> Mat4 {
        nalgebra_glm::perspective(self.aspect, f32::to_radians(self.fov), self.near, self.far)
    }

    pub(crate) fn set_aspect(&mut self, w: f32, h: f32) {
        self.aspect = w / h;
    }
}

impl Inspectable for Camera {
    fn get_object_ui(&mut self, ui: &Ui) {
        self.transform.default_ui(ui);
    }

    fn get_object_name(&self) -> String {
        "Camera".to_string()
    }
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::new_empty(),
            fov: 45.0,
            aspect: 800.0 / 600.0,
            near: 0.1,
            far: 100.0,
        }
    }
}
