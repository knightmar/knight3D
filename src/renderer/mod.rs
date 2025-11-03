use crate::scene::Scene;
use crate::shape::Shape;
use gl::types::GLuint;
use nalgebra_glm::Mat4;
use std::ffi::CString;
use std::sync::{Arc, Mutex};

pub mod color_renderer;

/// This enums is used to dynamically set uniforms
pub enum UniformValue {
    Float(f32),
    Int(i32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Matrix4fv(Mat4),
}

pub trait Renderer {
    fn init_buffers(&mut self);

    fn render(&self);
    fn get_obj_list(&self) -> Vec<Shape>;

    /// This method takes the name of the shaders and compiles them into a program shader before storing it in the struct's field
    /// # Arguments: names of the shaders
    /// # Effect: Returns a ColorRenderer completed with a new shader program
    fn init(vertex_shader_name: &str, fragment_shader_name: &str, scene: Arc<Mutex<Scene>>) -> Self;

    fn get_shader_program(&self) -> GLuint;

    /// This method is used to set a uniform based on the name and a value
    /// (Uniforms are a type of variable in opengl's shaders)
    fn set_uniform(&self, name: &'static str, value: UniformValue) {
        let shader_program = self.get_shader_program();

        unsafe {
            let i = gl::GetUniformLocation(shader_program, CString::new(name).unwrap().as_ptr());
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
