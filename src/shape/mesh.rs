use gl::types::{GLsizei, GLsizeiptr, GLuint};
use std::ffi::c_void;
use std::ptr::null;

#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coords: [f32; 2],
    pub color: [f32; 3]
}
pub struct MeshData {
    pub vertices: Vec<Vertex>,
    pub indices: Option<Vec<u32>>,
}

pub struct MeshGPU {
    pub vao: GLuint,
    pub vbo: GLuint,
    pub ebo: Option<GLuint>,
    pub index_count: u32,
}

impl MeshGPU {
    pub fn init(&self, mesh_data: MeshData) {
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
                (mesh_data.vertices.len() * size_of_val(&mesh_data.vertices[0])) as GLsizeiptr,
                mesh_data.vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // EBO
            if let Some(indices) = mesh_data.indices.clone() {
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
    }
}
