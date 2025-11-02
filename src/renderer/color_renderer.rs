use crate::renderer::Renderer;
use crate::shader::Shader;
use crate::shape::Shape;
use gl::types::{GLsizei, GLsizeiptr, GLuint};
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use std::ffi::c_void;
use std::ptr::null;

struct ColorRenderer<'a> {
    shader_program: GLuint,
    objects: Vec<Shape<'a>>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl<'a> Renderer<'a> for ColorRenderer<'a> {
    /// This method is called each frame, it binds the vertex and draws the shape

    fn init_buffers(&mut self) {
        let vertices: Vec<&Box<[([f32; 3], [f32; 3], [f32; 2])]>> =
            self.objects.iter().map(|x| x.vertices()).collect();
        let indices: Vec<&[u32]> =
            self.objects.iter().map(|x| x.indices().unwrap()).collect();

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            // VAO
            gl::BindVertexArray(self.vao);

            // VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of_val(&vertices[0])) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // EBO
            if let Some(indices) = indices {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
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
    }

    fn render(&self) {




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

    fn get_obj_list(&self) -> Vec<Shape<'a>> {
        self.objects.clone()
    }

    /// This method takes the name of the shaders and compiles them into a program shader before storing it in the struct's field
    /// # Arguments: names of the shaders
    /// # Effect: Returns a ColorRenderer completed with a new shader program
    fn init(vertex_shader_name: &str, fragment_shader_name: &str) -> Self {
        let vertex_shader = Shader::new(vertex_shader_name)
            .unwrap()
            .init_shader(VERTEX_SHADER);
        let fragment_shader = Shader::new(fragment_shader_name)
            .unwrap()
            .init_shader(FRAGMENT_SHADER);

        let program = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
            program
        };

        ColorRenderer {
            shader_program: program,
            objects: vec![],
        }
    }
}
