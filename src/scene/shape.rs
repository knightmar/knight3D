use crate::scene::object::{Object, Transform};
use crate::shader::Shader;
use crate::texture::Texture;
use crate::ui::Inspectable;
use gl::types::*;
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use imgui::Ui;
use nalgebra_glm::Mat4;
use std::ffi::{c_void, CString};
use std::ptr::null;

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
    vertices: &'a [([f32; 3], [f32; 3], [f32; 2])],
    indices: Option<&'a [u32]>,
    shader_program: GLuint,
    texture: Texture,
    pub transform: Transform,
}

impl<'a> Object for Shape<'a> {
    fn get_matrix(&self) -> Mat4 {
        self.transform.get_matrix()
    }
}

impl<'a> Inspectable for Shape<'a> {
    fn get_object_ui(&self, ui: &mut Ui) {
        self.transform.default_ui(ui);
    }
}

/// This enums is used to dynamically set uniforms
pub enum UniformValue {
    Float(f32),
    Int(i32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Matrix4fv(Mat4),
}

impl Shape<'_> {
    pub fn new<'a>(
        vertices: &'a [([f32; 3], [f32; 3], [f32; 2])],
        indices: Option<&'a [u32]>,
        texture_path: &str,
    ) -> Shape<'a> {
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
                (vertices.len() * size_of_val(&vertices[0])) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // EBO
            if let Some(indices) = indices {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * size_of::<u32>()) as GLsizeiptr,
                    indices.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }

            // position attrib
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * size_of::<f32>()) as GLsizei,
                null(),
            );
            gl::EnableVertexAttribArray(0);

            // color attrib
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * size_of::<f32>()) as GLsizei,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // texture attrib
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (8 * size_of::<f32>()) as GLsizei,
                (6 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
        }

        let texture = Texture::new(texture_path).unwrap();

        Shape {
            vao,
            vbo,
            ebo: if ebo == 0 { None } else { Some(ebo) },
            vertices,
            indices,
            shader_program: 0,
            texture,
            transform: Transform::new_empty(),
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
    pub fn render(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.texture_id);
            gl::BindVertexArray(self.vao);
            if self.indices.is_some() {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.indices.unwrap().len() as GLsizei,
                    gl::UNSIGNED_INT,
                    null(),
                );
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as GLsizei)
            }

            gl::BindVertexArray(0);
        }
    }

    /// This method is used to set a uniform based on the name and a value
    /// (Uniforms are a type of variable in opengl's shaders)
    pub fn set_uniform(&self, name: &'static str, value: UniformValue) {
        unsafe {
            let i =
                gl::GetUniformLocation(self.shader_program, CString::new(name).unwrap().as_ptr());
            gl::UseProgram(self.shader_program);
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
