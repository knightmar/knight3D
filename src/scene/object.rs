use imgui::Ui;
use nalgebra_glm::{Mat4, Quat};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub(crate) position: [f32; 3],
    pub(crate) rotation: Quat,
    pub(crate) rotation_ui: [f32; 4],
    pub(crate) scale: [f32; 3],
}

impl Transform {
    pub fn translate(&mut self, delta: [f32; 3]) {
        self.position = [
            self.position[0] + delta[0],
            self.position[1] + delta[1],
            self.position[2] + delta[2],
        ];
    }

    pub fn rotate(&mut self, axis: [f32; 3], angle: f32) {
        let q =
            nalgebra_glm::quat_angle_axis(f32::to_radians(angle), &nalgebra_glm::make_vec3(&axis));
        self.rotation = q * self.rotation;
    }

    pub(crate) fn get_matrix(&self) -> Mat4 {
        let translation = nalgebra_glm::translation(&nalgebra_glm::make_vec3(&self.position));
        let rotation =
            nalgebra_glm::quat_to_mat4(&nalgebra_glm::make_quat(&self.rotation.coords.as_slice()));
        let scale = nalgebra_glm::scaling(&nalgebra_glm::make_vec3(&self.scale));

        translation * rotation * scale
    }

    pub fn new(position: [f32; 3], rotation: nalgebra_glm::Quat, scale: [f32; 3]) -> Self {
        Self {
            position,
            rotation,
            scale,
            rotation_ui: rotation.as_vector().into(),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: Quat::identity(),
            scale: [1.0, 1.0, 1.0],
            rotation_ui: Quat::identity().as_vector().into(),
        }
    }

    pub fn default_ui(&mut self, ui: &Ui) {
        // Use hidden ID to avoid collisions within same scope if needed
        ui.input_float3("Position##transform", &mut self.position).build();

        ui.input_float4("Rotation##transform", &mut self.rotation_ui).build();
        if ui.button("Apply rotation") {
            self.set_rotation(Quat::from(self.rotation_ui));
        }

    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: [f32; 3]) {
        self.scale = scale;
    }
}
pub trait Object {
    fn get_matrix(&self) -> Mat4;
}
