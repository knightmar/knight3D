mod directional_light;

use crate::objects::shape::{Shape, UniformValue};
use crate::objects::{Renderable, Transform};
use gl::types::GLuint;

pub enum LightType {
    Directional,
    Point,
    Spot,
}
trait Light {
    fn get_type(&self) -> LightType;
    fn get_transform(&self) -> Transform;
}

struct Lighting {
    lights: Vec<Box<dyn Light>>,
    pub shader_program: GLuint,
}

impl Lighting {
    pub fn upload_lights(&self, shape: Shape) {
        for (i, light) in self.lights.iter().clone().enumerate() {
            shape.set_uniform(
                format_args!("pointLights[{}].position", i).to_string(),
                UniformValue::Vec3(light.get_transform().position),
            );
        }
    }
}
