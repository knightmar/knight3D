use crate::objects::light::{Light, LightType};
use crate::objects::Transform;

pub struct PointLight {
    pub name: String,
    pub transform: Transform,
    pub color: [f32; 3]
}

impl Light for PointLight {
    fn get_type(&self) -> LightType {
        LightType::Directional
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }

    fn get_color(&self) -> [f32; 3] {
        self.color
    }
}
