pub mod directional_light;
pub mod point_light;

use crate::objects::light::directional_light::DirectionalLight;
use crate::objects::light::point_light::PointLight;
use crate::objects::shape::{Shape, UniformValue};
use crate::objects::{Renderable, Transform};
use nalgebra::UnitQuaternion;

pub enum LightType {
    Directional,
    Point,
    Spot,
}
trait Light {
    fn get_type(&self) -> LightType;
    fn get_transform(&self) -> Transform;
}

pub struct Lighting {
    pub dir_light: Box<DirectionalLight>,
    pub point_lights: Vec<PointLight>,
}

impl Lighting {
    pub fn upload_lights(&self, shape: &Shape) {
        shape.set_uniform(
            "lightsCount".to_string(),
            UniformValue::Int(self.point_lights.len() as i32),
        );
        for (i, light) in self.point_lights.iter().clone().enumerate() {
            shape.set_uniform(
                format!("pointLights[{}].position", i).to_string(),
                UniformValue::Vec3(light.get_transform().position),
            );

            shape.set_uniform(
                format!("pointLights[{}].ambient", i).to_string(),
                UniformValue::Vec3(light.ambient),
            );

            shape.set_uniform(
                format!("pointLights[{}].diffuse", i).to_string(),
                UniformValue::Vec3(light.diffuse),
            );

            shape.set_uniform(
                format!("pointLights[{}].specular", i).to_string(),
                UniformValue::Vec3(light.specular),
            );

            shape.set_uniform(
                format!("pointLights[{}].constant", i).to_string(),
                UniformValue::Float(light.constant),
            );

            shape.set_uniform(
                format!("pointLights[{}].linear", i).to_string(),
                UniformValue::Float(light.linear),
            );

            shape.set_uniform(
                format!("pointLights[{}].quadratic", i).to_string(),
                UniformValue::Float(light.quadratic),
            );
        }

        let direction = UnitQuaternion::from_quaternion(self.dir_light.get_transform().rotation)
            .transform_vector(&nalgebra_glm::vec3(0.0, 0.0, -1.0));

        shape.set_uniform(
            "dirLight.direction".to_string().to_string(),
            UniformValue::Vec3(<[f32; 3]>::try_from(direction.data.as_slice()).unwrap()),
        );

        shape.set_uniform(
            "dirLight.ambient".to_string().to_string(),
            UniformValue::Vec3(self.dir_light.ambient),
        );

        shape.set_uniform(
            "dirLight.diffuse".to_string().to_string(),
            UniformValue::Vec3(self.dir_light.diffuse),
        );

        shape.set_uniform(
            "dirLight.specular".to_string().to_string(),
            UniformValue::Vec3(self.dir_light.specular),
        );
    }
}
