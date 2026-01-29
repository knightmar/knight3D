use gl::types::GLuint;
use crate::objects::light::{Light, LightType};
use crate::objects::{Renderable, Transform};
use crate::objects::shape::UniformValue;

struct DirectionalLight {
    pub name: String,
    pub shader_program: GLuint,
    pub transform: Transform,
}

impl Light for DirectionalLight {
    fn get_type(&self) -> LightType {
        LightType::Directional
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }
}

impl Renderable for DirectionalLight {
    fn get_program_id(&self) -> u32 {
        todo!()
    }

    fn render(&self) {
        todo!()
    }

    fn init_shaders(&mut self, vertex_shader_name: &str, fragment_shader_name: &str) {
        todo!()
    }

    fn set_uniform(&self, name: String, value: UniformValue) {
        todo!()
    }
}