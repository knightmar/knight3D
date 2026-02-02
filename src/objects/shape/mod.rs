mod mesh;
pub mod obj_parsing;

use crate::objects::shape::mesh::{Mesh, MeshData, MeshGPU, Vertex};
use crate::objects::{Renderable, Transform};
use crate::shader::Shader;
use crate::texture::Texture;
use crate::ui::Inspectable;
use gl::types::*;
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use imgui::Ui;
use nalgebra_glm::Mat4;
use std::ptr::null;
use std::rc::Rc;

/// This struct represents a shape that will be rendered.
/// # Fields :
/// - vao / vbo / ebo : all the buffers of the shape (read about them in opengl doc)
/// - vertices : the list of tuple holding a 3D point + color : `([x, y, z], [r, g, b])`
/// - indices : the list holding the vertices needed to be drawn with the help of the ebo
/// - shader_program : the index of the shader program that will be linked when the shaders are compiled in the init_shaders method
#[derive(Clone)]
pub struct Shape {
    pub name: String,
    mesh: Rc<Mesh>,
    pub texture: Texture,
    pub shader_program: GLuint,
    pub transform: Transform,
}

impl Renderable for Shape {
    fn get_program_id(&self) -> u32 {
        self.shader_program
    }

    fn render(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
            gl::BindTexture(gl::TEXTURE_2D, self.texture.texture_id);
            gl::BindVertexArray(self.mesh.mesh_gpu.vao);
            if self.mesh.mesh_gpu.index_count > 0 {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.mesh.mesh_gpu.index_count as GLsizei,
                    gl::UNSIGNED_INT,
                    null(),
                );
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.mesh.mesh_gpu.index_count as GLsizei)
            }

            gl::BindVertexArray(0);
        }
    }

    fn init_shaders(&mut self, vertex_shader_name: &str, fragment_shader_name: &str) {
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
}

impl Inspectable for Shape {
    fn get_object_ui(&mut self, ui: &Ui) {
        self.transform.default_ui(ui, self.get_object_name());
    }

    fn get_object_name(&self) -> String {
        self.name.clone()
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

impl Shape {
    pub fn new(
        name: String,
        data: Box<[([f32; 3], [f32; 3], [f32; 2], [f32; 3])]>, // pos : color : tex_pos : normal
        indices: Option<Vec<u32>>,
        texture_path: &str,
    ) -> Shape {
        let mut vertices = Vec::<Vertex>::new();
        data.iter()
            .for_each(|(position, color, tex_coords, normal)| {
                vertices.push(Vertex {
                    position: *position,
                    color: *color,
                    normal: *normal,
                    tex_coords: *tex_coords,
                });
            });

        Self::new_from_vertex(name, vertices, indices, texture_path)
    }

    pub fn new_from_vertex(
        name: String,
        data: Vec<Vertex>,
        indices: Option<Vec<u32>>,
        texture_path: &str,
    ) -> Shape {
        let texture = Texture::new(texture_path).unwrap();

        let mesh_data = MeshData {
            vertices: data,
            indices: indices.clone(),
        };

        let mut mesh_gpu = MeshGPU::default();
        mesh_gpu.init(mesh_data.clone());

        let mesh = Mesh {
            mesh_data,
            mesh_gpu,
        };

        Shape {
            name,
            mesh: Rc::from(mesh),
            shader_program: 0,
            texture,
            transform: Transform::new_empty(),
        }
    }

    pub fn mesh(&self) -> &Mesh {
        &self.mesh
    }

    pub fn set_mesh(&mut self, mesh_data: MeshData) {
        let mut mesh_gpu = MeshGPU::default();
        mesh_gpu.init(mesh_data.clone());
        let mesh = Mesh {
            mesh_data,
            mesh_gpu,
        };
        self.mesh = Rc::new(mesh);
    }
}
