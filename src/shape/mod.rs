mod mesh;
pub mod obj_parsing;

use crate::scene::object::{Object, Transform};
use crate::shader::Shader;
use crate::shape::mesh::{MeshData, MeshGPU, Vertex};
use crate::texture::Texture;
use crate::ui::Inspectable;
use gl::types::*;
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use imgui::Ui;
use nalgebra_glm::Mat4;
use std::ffi::CString;
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
    pub mesh: Rc<MeshGPU>,
    pub texture: Texture,
    pub shader_program: GLuint,
    pub transform: Transform,
}

impl Object for Shape {
    fn get_matrix(&self) -> Mat4 {
        self.transform.get_matrix()
    }
}

impl Inspectable for Shape {
    fn get_object_ui(&mut self, ui: &Ui) {
        self.transform.default_ui(ui);
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
        data: Box<[([f32; 3], [f32; 3], [f32; 2])]>, // pos : color : tex_pos
        indices: Option<Vec<u32>>,
        texture_path: &str,
    ) -> Shape {
        let texture = Texture::new(texture_path).unwrap();
        let mut vertices = Vec::<Vertex>::new();
        data.iter().for_each(|(position, color, tex_coords)| {
            vertices.push(Vertex {
                position: *position,
                color: *color,
                // normal: [0.0, 0.0, 0.0],
                tex_coords: *tex_coords,
            });
        });

        let mesh_data = MeshData {
            vertices,
            indices: indices.clone(),
        };

        let mut mesh = MeshGPU {
            vao: 0,
            vbo: 0,
            ebo: None,
            index_count: indices.iter().count() as u32,
        };

        mesh.init(mesh_data);

        Shape {
            name,
            mesh: Rc::from(mesh),
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
            gl::BindVertexArray(self.mesh.vao);
            if self.mesh.index_count > 0 {
                gl::DrawElements(
                    gl::TRIANGLES,
                    self.mesh.index_count as GLsizei,
                    gl::UNSIGNED_INT,
                    null(),
                );
            } else {
                gl::DrawArrays(gl::TRIANGLES, 0, self.mesh.index_count as GLsizei)
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
