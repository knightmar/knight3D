use crate::scene::Scene;
use crate::ui::Inspectable;
use imgui::{Condition, Ui, Window, WindowFlags};
use std::sync::{Arc, Mutex};

pub fn left_panel_ui<'a>(ui: &mut Ui, scene: &Arc<Mutex<Scene<'a>>>, test: &mut i32, fixed_h: f32) {
    Window::new(&ui, "Objects")
        .position([0.0, 0.0], Condition::Always)
        .size_constraints([2.0, fixed_h], [f32::MAX, fixed_h])
        .size([300.0, fixed_h], Condition::FirstUseEver)
        .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
        .build(|| {
            let window_ui = &ui;

            if ui.button("test") {
                *test += 1;
            }

            if ui.input_int("test value", test).build() {
                println!("edited")
            };

            let scene = &mut scene.lock().unwrap();
            let mut ui_items: Vec<Box<Inspectable>> = Vec::from(Box::new(scene.shapes));
            ui_items.push(Box::new(scene.camera));

            for mut item in ui_items {
                item.get_object_ui(*window_ui);
                ui.separator()
            }
        });
}
