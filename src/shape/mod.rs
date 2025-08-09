use crate::shape::shaders::Shader;
use gl::types::*;
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use std::ffi::{c_void, CString};
use std::ptr::null;

pub mod shaders;

/// This struct represents a shape that will be rendered.
/// # Fields :
/// - vao / vbo / ebo : all the buffers of the shape (read about them in opengl doc)
/// - vertices : the list of tuple holding a 3D point + color : `([x, y, z], [r, g, b])`
/// - indices : the list holding the vertices needed to be drawn with the help of the ebo
/// - shader_program : the index of the shader program that will be linked when the shaders are compiled in the init_shaders method
pub struct Shape<'a> {
    vao: GLuint,
    vbo: GLuint,
    ebo: Option<GLuint>,
    vertices: &'a [([f32; 3], [f32; 3])],
    indices: &'a [u32],
    shader_program: GLuint,
}

/// This enums is used to dynamically set uniforms
pub enum UniformValue {
    Float(f32),
    Int(i32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
}

impl Shape<'_> {
    pub fn new<'a>(vertices: &'a [([f32; 3], [f32; 3])], indices: &'a [u32]) -> Shape<'a> {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            // VAO
            gl::BindVertexArray(vao);

            // VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<([f32; 3], [f32; 3])>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // EBO
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as GLsizeiptr,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Attrib pointers
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * size_of::<f32>()) as GLsizei,
                null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (6 * size_of::<f32>()) as GLsizei,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }
        Shape {
            vao,
            vbo,
            ebo: Some(ebo),
            vertices,
            indices,
            shader_program: 0,
        }
    }

    /// This method takes the name of the shaders and compiles them into a program shader before storing it in the struct's field
    /// # Arguments : names of the shaders
    /// # Effect : Updates the shader_program field of the struct
    pub fn init_shaders(&mut self, vertex_shader_name: &str, fragment_shader_name: &str) {
        let vertex_shader = Shader::new(vertex_shader_name)
            .unwrap()
            .init_shader(VERTEX_SHADER);
        let fragment_shader = Shader::new(fragment_shader_name)
            .unwrap()
            .init_shader(FRAGMENT_SHADER);

        self.shader_program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            program
        };
    }

    /// This method is called each frame, it binds the vertex and draws the shape
    pub unsafe fn render(&self) {
        gl::UseProgram(self.shader_program);
        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as GLsizei,
            gl::UNSIGNED_INT,
            null(),
        );
        gl::BindVertexArray(0);
    }

    /// This method is used to set a uniform based on the name and a value
    /// (Uniforms are a type of variable in opengl's shaders)
    pub unsafe fn set_uniform(&self, name: &'static str, value: UniformValue) {
        let i = gl::GetUniformLocation(self.shader_program, CString::new(name).unwrap().as_ptr());
        gl::UseProgram(self.shader_program);
        match value {
            UniformValue::Float(v) => gl::Uniform1f(i, v),
            UniformValue::Int(v) => gl::Uniform1i(i, v),
            UniformValue::Vec2(v) => gl::Uniform2f(i, v[0], v[1]),
            UniformValue::Vec3(v) => gl::Uniform3f(i, v[0], v[1], v[2]),
            UniformValue::Vec4(v) => gl::Uniform4f(i, v[0], v[1], v[2], v[3]),
        }
    }
}
