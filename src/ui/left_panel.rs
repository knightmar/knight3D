use crate::scene::Scene;
use crate::ui::Inspectable;
use imgui::{Condition, Ui, Window, WindowFlags};
use std::sync::{Arc, Mutex};

pub fn left_panel_ui<'a>(ui: &mut Ui, scene: &Arc<Mutex<Scene<'a>>>, fps: u32, fixed_h: f32) {
    Window::new(&ui, "Objects")
        .position([0.0, 0.0], Condition::Always)
        .size_constraints([2.0, fixed_h], [f32::MAX, fixed_h])
        .size([300.0, fixed_h], Condition::FirstUseEver)
        .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
        .build(|| {
            let window_ui = &ui;
            let scene = &mut scene.lock().unwrap();

            ui.text("Infos:");
            ui.text(fps.to_string());

            ui.separator();
            ui.text("Object list:");

            let _id = ui.push_id("camera");
            ui.text(scene.camera.get_object_name());
            scene.camera.get_object_ui(*window_ui);
            ui.separator();

            for (i, shape) in scene.shapes.iter_mut().enumerate() {
                let _id = ui.push_id_usize(i);
                ui.text(shape.get_object_name());
                shape.get_object_ui(*window_ui);
                ui.separator();
            }
        });
}
