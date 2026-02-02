pub mod directional_light;
mod point_light;

use crate::objects::light::directional_light::DirectionalLight;
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
}

impl Lighting {
    pub fn upload_lights(&self, shape: &Shape) {
        // for (i, light) in self.lights.iter().clone().enumerate() {
        //     shape.set_uniform(
        //         format_args!("pointLights[{}].position", i).to_string(),
        //         UniformValue::Vec3(light.get_transform().position),
        //     );
        //     let direction = UnitQuaternion::from_quaternion(light.get_transform().rotation)
        //         .transform_vector(&nalgebra_glm::vec3(0.0, 0.0, -1.0));
        //
        //     shape.set_uniform(
        //         format_args!("pointLights[{}].direction", i).to_string(),
        //         UniformValue::Vec3(<[f32; 3]>::try_from(direction.data.as_slice()).unwrap()),
        //     );
        // }

        let direction = UnitQuaternion::from_quaternion(self.dir_light.get_transform().rotation)
            .transform_vector(&nalgebra_glm::vec3(0.0, 0.0, -1.0));

        shape.set_uniform(
            format_args!("dirLight.direction").to_string(),
            UniformValue::Vec3(<[f32; 3]>::try_from(direction.data.as_slice()).unwrap()),
        );

        shape.set_uniform(
            format_args!("dirLight.ambient").to_string(),
            UniformValue::Vec3(self.dir_light.ambient),
        );

        shape.set_uniform(
            format_args!("dirLight.diffuse").to_string(),
            UniformValue::Vec3(self.dir_light.diffuse),
        );

        shape.set_uniform(
            format_args!("dirLight.specular").to_string(),
            UniformValue::Vec3(self.dir_light.specular),
        );
    }
}
