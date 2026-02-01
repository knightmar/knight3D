pub mod shape;
pub mod light;

use std::ffi::CString;
use crate::objects::shape::UniformValue;
use crate::utils::{euler_deg_from_quat, quat_from_euler_deg};
use imgui::Ui;
use nalgebra_glm::{Mat4, Quat};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub(crate) position: [f32; 3],
    pub(crate) rotation: Quat,
    pub(crate) rotation_ui: [f32; 3],
    pub(crate) rotation_ui_editing: bool,
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

    pub fn new(position: [f32; 3], rotation: Quat, scale: [f32; 3]) -> Self {
        Self {
            position,
            rotation,
            scale,
            rotation_ui: [0.0, 0.0, 0.0],
            rotation_ui_editing: false,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: Quat::identity(),
            scale: [1.0, 1.0, 1.0],
            rotation_ui: [0.0, 0.0, 0.0],
            rotation_ui_editing: false,
        }
    }

    pub fn default_ui(&mut self, ui: &Ui) {
        ui.input_float3("Position##transform", &mut self.position)
            .build();

        if !self.rotation_ui_editing {
            self.rotation_ui = euler_deg_from_quat(self.rotation);
        }

        let changed = ui
            .input_float3("Rotation##transform", &mut self.rotation_ui)
            .build();

        let active = ui.is_item_active();
        self.rotation_ui_editing = active;

        if changed {
            self.rotation = quat_from_euler_deg(self.rotation_ui);
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
pub trait Renderable {
    fn get_program_id(&self) -> u32;

    /// This method is called each frame, it binds the vertex and draws the shape
    fn render(&self);
    /// This method takes the name of the shaders and compiles them into a program shader before storing it in the struct's field
    /// # Arguments : names of the shaders
    /// # Effect : Updates the shader_program field of the struct
    fn init_shaders(&mut self, vertex_shader_name: &str, fragment_shader_name: &str);
    /// This method is used to set a uniform based on the name and a value
    /// (Uniforms are a type of variable in opengl's shaders)
    fn set_uniform(&self, name: String, value: UniformValue) {
        unsafe {
            let shader_program = self.get_program_id();
            let i =
                gl::GetUniformLocation(shader_program, CString::new(name).unwrap().as_ptr());
            gl::UseProgram(shader_program);
            match value {
                UniformValue::Float(v) => gl::Uniform1f(i, v),
                UniformValue::Int(v) => gl::Uniform1i(i, v),
                UniformValue::Vec2(v) => gl::Uniform2f(i, v[0], v[1]),
                UniformValue::Vec3(v) => gl::Uniform3f(i, v[0], v[1], v[2]),
                UniformValue::Vec4(v) => gl::Uniform4f(i, v[0], v[1], v[2], v[3]),
                UniformValue::Matrix4fv(v) => {
                    gl::UniformMatrix4fv(i, 1, gl::FALSE, v.data.as_slice().as_ptr())
                }
            }
        }
    }
}
