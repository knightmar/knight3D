use imgui::Ui;
use nalgebra_glm::Vec3;
use crate::objects::light::{Light, LightType};
use crate::objects::Transform;
use crate::ui::Inspectable;

pub struct DirectionalLight {
    pub name: String,
    pub transform: Transform,

    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],
}

impl Light for DirectionalLight {
    fn get_type(&self) -> LightType {
        LightType::Directional
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }
}


impl Inspectable for DirectionalLight {
    fn get_object_ui(&mut self, ui: &Ui) {
        self.transform.default_ui(ui);
        ui.input_float3("Ambient", &mut self.ambient).build();
        ui.input_float3("Diffuse", &mut self.diffuse).build();
        ui.input_float3("Specular", &mut self.specular).build();
    }

    fn get_object_name(&self) -> String {
        self.name.clone()
    }
}