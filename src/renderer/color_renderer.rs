use crate::renderer::{Renderer, UniformValue};
use crate::scene::object::Object;
use crate::scene::Scene;
use crate::shader::Shader;
use crate::shape::Shape;
use gl::types::{GLsizei, GLsizeiptr, GLuint};
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use nalgebra_glm::Mat4;
use std::ffi::c_void;
use std::ptr::null;
use std::sync::{Arc, Mutex};

pub struct ColorRenderer {
    shader_program: GLuint,
    vao: GLuint,
    data_vbo: GLuint,
    model_matrix_vbo: GLuint,
    ebo: GLuint,
    vertices_count: u32,
    indices_count: u32,
    scene: Arc<Mutex<Scene>>,
}

impl Renderer for ColorRenderer {
    fn init_buffers(&mut self) {
        let obj_list = self.get_obj_list();
        let indices: Option<Vec<Vec<u32>>> = obj_list.iter().map(|x| x.indices()).collect();

        let positions_refs: Vec<&Box<[[f32; 3]]>> = obj_list.iter().map(|x| x.vertices()).collect();
        let colors_refs: Vec<&Box<[[f32; 3]]>> = obj_list.iter().map(|x| x.colors()).collect();
        let indices_refs: Vec<Option<Vec<u32>>> = obj_list.iter().map(|x| x.indices()).collect();

        let mut all_interleaved_data: Vec<f32> = Vec::new();
        let mut all_indices: Vec<u32> = Vec::new();
        let mut current_vertex_offset = 0;

        for (i, shape) in obj_list.iter().enumerate() {
            let shape_positions = &positions_refs[i];
            let shape_colors = &colors_refs[i];
            let shape_indices = indices_refs[i].clone();

            let num_shape_vertices = shape_positions.len();

            for v_idx in 0..num_shape_vertices {
                all_interleaved_data.extend_from_slice(&shape_positions[v_idx]);
                all_interleaved_data.extend_from_slice(&shape_colors[v_idx]);
            }

            if let Some(indices) = shape_indices {
                for idx in indices {
                    all_indices.push(idx + current_vertex_offset as u32);
                }
            }
            current_vertex_offset += num_shape_vertices;
        }

        self.indices_count = all_indices.len() as u32;
        self.vertices_count = current_vertex_offset as u32;

        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.data_vbo);
            gl::GenBuffers(1, &mut self.model_matrix_vbo);
            gl::GenBuffers(1, &mut self.ebo);

            // VAO
            gl::BindVertexArray(self.vao);

            // DATA VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.data_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (all_interleaved_data.len() * size_of::<f32>()) as GLsizeiptr,
                all_interleaved_data.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // EBO
            if let Some(indices) = indices {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (all_indices.len() * size_of::<u32>()) as GLsizeiptr,
                    all_indices.as_ptr() as *const _,
                    gl::STATIC_DRAW,
                );
            }

            let stride_per_vertex = (3 + 3) * size_of::<f32>(); // 6 floats (pos + color)

            // position attrib (location = 0)
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride_per_vertex as GLsizei,
                null(),
            );
            gl::EnableVertexAttribArray(0);

            // color attrib (location = 1)
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride_per_vertex as GLsizei,
                (3 * size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // MATRIX VBO
            gl::BindBuffer(gl::ARRAY_BUFFER, self.model_matrix_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (obj_list.len() * size_of::<Mat4>()) as GLsizeiptr,
                null(),
                gl::DYNAMIC_DRAW, // Matrices will change
            );

            let mat4_size = size_of::<Mat4>();
            let vec4_size = size_of::<nalgebra_glm::Vec4>();

            let first_mat4_attrib_location = 2;

            for i in 0..4 {
                gl::EnableVertexAttribArray(first_mat4_attrib_location + i as GLuint);
                gl::VertexAttribPointer(
                    first_mat4_attrib_location + i as GLuint,
                    4,
                    gl::FLOAT,
                    gl::FALSE,
                    mat4_size as GLsizei,
                    (i * vec4_size) as *const gl::types::GLvoid,
                );
                gl::VertexAttribDivisor(first_mat4_attrib_location + i as GLuint, 1);
            }

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); // Unbind VBOs
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0); // Unbind EBO
        }
    }

    /// This method is called each frame, it binds the vertex and draws the shape
    fn render(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);

            let scene_guard = self.scene.lock().unwrap();
            let obj_list = &scene_guard.shapes;

            let model_matrices: Vec<Mat4> = obj_list.iter().map(|s| s.get_matrix()).collect();
            gl::BindBuffer(gl::ARRAY_BUFFER, self.model_matrix_vbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (model_matrices.len() * size_of::<Mat4>()) as GLsizeiptr,
                model_matrices.as_ptr() as *const gl::types::GLvoid,
            );
            // Pas besoin de délier ici, car le VAO gère les liaisons d'attributs

            let view_matrix = scene_guard.camera.get_view_matrix();
            let projection_matrix = scene_guard.camera.get_projection_matrix();

            self.set_uniform("view", UniformValue::Matrix4fv(view_matrix));
            self.set_uniform("projection", UniformValue::Matrix4fv(projection_matrix));

            if self.indices_count > 0 {
                gl::DrawElementsInstanced(
                    gl::TRIANGLES,
                    self.indices_count as i32,
                    gl::UNSIGNED_INT,
                    null(),
                    obj_list.len() as i32,
                );
            } else {
                gl::DrawArraysInstanced(
                    gl::TRIANGLES,
                    0,
                    self.vertices_count as i32,
                    obj_list.len() as i32,
                );
            }

            gl::BindVertexArray(0);
        }
    }

    fn get_obj_list(&self) -> Vec<Shape> {
        self.scene.lock().unwrap().shapes.clone()
    }

    /// This method takes the name of the shaders and compiles them into a program shader before storing it in the struct's field
    /// # Arguments: names of the shaders
    /// # Effect: Returns a ColorRenderer completed with a new shader program
    fn init(
        vertex_shader_name: &str,
        fragment_shader_name: &str,
        scene: Arc<Mutex<Scene>>,
    ) -> Self {
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
            vao: 0,
            data_vbo: 0,
            model_matrix_vbo: 0,
            ebo: 0,
            vertices_count: 0,
            indices_count: 0,
            scene,
        }
    }

    fn get_shader_program(&self) -> GLuint {
        self.shader_program
    }
}
