use crate::shape::shaders::Shader;
use gl::types::*;
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use std::ptr::null;

pub mod shaders;

pub struct Shape<'a> {
    vao: GLuint,
    vbo: GLuint,
    ebo: Option<GLuint>,
    vertices: &'a [[f32; 3]],
    indices: &'a [u32; 6],
    shader_program: GLuint,
}

impl Shape<'_> {
    pub fn new<'a>(vertices: &'a[[f32; 3]], indices: &'a[u32; 6]) -> Shape<'a> {
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
                (vertices.len() * size_of::<[f32; 3]>()) as GLsizeiptr,
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
                (3 * size_of::<f32>()) as GLsizei,
                null(),
            );
            gl::EnableVertexAttribArray(0);
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
}
