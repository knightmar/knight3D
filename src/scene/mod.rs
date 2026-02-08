use crate::objects::light::directional_light::DirectionalLight;
use crate::objects::light::Lighting;
use crate::objects::shape::{Shape, UniformValue};
use crate::objects::{Renderable, Transform};
use crate::scene::camera::Camera;
use crate::TIME;

pub mod camera;

pub struct Scene {
    pub shapes: Vec<Shape>,
    pub camera: Camera,
    pub lighting: Lighting,
}

impl<'a> Scene {
    pub fn new() -> Scene {
        Scene {
            shapes: vec![],
            camera: Camera::new(),
            lighting: Lighting {
                dir_light: Box::from(DirectionalLight {
                    name: "DirLight".to_string(),
                    transform: Transform::new_empty(),
                    ambient: [1.0, 1.0, 1.0],
                    diffuse: [1.0, 1.0, 1.0],
                    specular: [1.0, 1.0, 1.0],
                }),
                point_lights: vec![],
            },
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }
    pub fn remove_shape(&mut self, i: u32) {
        self.shapes.remove(i as usize);
    }

    pub fn render(&mut self) {
        self.lighting.dir_light.transform.rotate([1.0, 1.0, 0.0], 2.0);

        for shape in &self.shapes {
            shape.set_uniform(
                "model".to_string(),
                UniformValue::Matrix4fv(shape.transform.get_matrix()),
            );
            shape.set_uniform(
                "view".to_string(),
                UniformValue::Matrix4fv(self.camera.get_view_matrix()),
            );
            shape.set_uniform(
                "projection".to_string(),
                UniformValue::Matrix4fv(self.camera.get_projection_matrix()),
            );

            shape.set_uniform(
                "viewPos".to_string(),
                UniformValue::Vec3(self.camera.transform.position),
            );

            unsafe {
                shape.set_uniform("time".to_string(), UniformValue::Float(TIME as f32));
            }
            self.lighting.upload_lights(shape);
            shape.render();
        }
    }
}
