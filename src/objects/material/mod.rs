use crate::objects::shape::{Shape, UniformValue};
use crate::objects::Renderable;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Material {
    pub ambient: Texture,
    pub specular: Option<Texture>,
    pub shininess: f32,
}

impl Material {
    pub fn new(ambient_path: &str, specular_path: &str, shininess: f32) -> Self {
        Self {
            ambient: Texture::new(ambient_path).unwrap(),
            specular: if !specular_path.is_empty() {
                Some(Texture::new(specular_path).unwrap())
            } else {
                None
            },
            shininess,
        }
    }

    pub fn bind(&self, shape: &Shape) {
        shape.set_uniform("material.ambient".to_string(), UniformValue::Int(0));
        shape.set_uniform(
            "material.shininess".to_string(),
            UniformValue::Float(self.shininess),
        );

        shape.set_uniform(
            "material.hasSpecularMap".to_string(),
            UniformValue::Int(i32::from(self.specular.is_some())),
        );

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.ambient.texture_id);

            if let Some(specular) = self.specular {
                shape.set_uniform("material.specular".to_string(), UniformValue::Int(1));
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, specular.texture_id);
            }
        }
    }
}
