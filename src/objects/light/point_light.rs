use crate::objects::Transform;
use crate::objects::light::{Light, LightType};
use crate::ui::Inspectable;
use imgui::Ui;

pub struct PointLight {
    pub name: String,
    pub transform: Transform,

    pub ambient: [f32; 3],
    pub diffuse: [f32; 3],
    pub specular: [f32; 3],

    pub constant: f32,
    pub linear: f32,
    pub quadratic: f32,
}

impl Light for PointLight {
    fn get_type(&self) -> LightType {
        LightType::Point
    }

    fn get_transform(&self) -> Transform {
        self.transform
    }
}

impl Inspectable for PointLight {
    fn get_object_ui(&mut self, ui: &Ui) {
        self.transform.default_ui(ui);
        ui.input_float3("Ambient##pointlight", &mut self.ambient).build();
        ui.input_float3("Diffuse##pointlight", &mut self.diffuse).build();
        ui.input_float3("Specular##pointlight", &mut self.specular).build();
        ui.input_float("Constant##pointlight", &mut self.constant).build();
        ui.input_float("Linear##pointlight", &mut self.linear).build();
        ui.input_float("Quadratic##pointlight", &mut self.quadratic).build();
    }

    fn get_object_name(&self) -> String {
        self.name.clone()
    }
}
