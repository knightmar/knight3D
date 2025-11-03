pub mod obj_parsing;

use crate::scene::object::{Object, Transform};
use crate::texture::Texture;
use crate::ui::Inspectable;
use imgui::Ui;
use nalgebra_glm::Mat4;

/// This struct represents a shape that will be rendered.
/// # Fields :
/// - vao / vbo / ebo : all the buffers of the shape (read about them in opengl doc)
/// - vertices : the list of tuple holding a 3D point + color : `([x, y, z], [r, g, b])`
/// - indices : the list holding the vertices needed to be drawn with the help of the ebo
/// - shader_program : the index of the shader program that will be linked when the shaders are compiled in the init_shaders method
#[derive(Clone)]
pub struct Shape {
    name: String,
    vertices: Box<[[f32; 3]]>,
    colors: Box<[[f32; 3]]>,
    tex_pos: Box<[[f32; 2]]>,
    indices: Option<Vec<u32>>,
    texture: Texture,
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

impl Shape {
    pub fn new(
        name: String,
        vertices: Box<[[f32; 3]]>,
        colors: Box<[[f32; 3]]>,
        tex_pos: Box<[[f32; 2]]>,
        indices: Option<Vec<u32>>,
        texture_path: &str,
    ) -> Shape {
        let texture = Texture::new(texture_path).unwrap();

        Shape {
            name,
            vertices,
            colors,
            tex_pos,
            indices,
            texture,
            transform: Transform::new_empty(),
        }
    }

    pub fn vertices(&self) -> &Box<[[f32; 3]]> {
        &self.vertices
    }

    pub fn colors(&self) -> &Box<[[f32; 3]]> {
        &self.colors
    }

    pub fn tex_pos(&self) -> &Box<[[f32; 2]]> {
        &self.tex_pos
    }

    pub fn indices(&self) -> Option<Vec<u32>> {
        self.indices.clone()
    }

    pub fn texture(&self) -> Texture {
        self.texture
    }
}
