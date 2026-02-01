use crate::scene::Scene;
use crate::ui::Inspectable;
use imgui::{Condition, StyleVar, Ui, Window, WindowFlags};
use rand::random;
use std::sync::{Arc, Mutex};

pub fn left_panel_ui<'a>(ui: &mut Ui, scene: &Arc<Mutex<Scene>>, fps: u32, fixed_h: f32) {
    let _pad = ui.push_style_var(StyleVar::WindowPadding([0.0, 0.0]));

    Window::new(&ui, "Objects")
        .position([0.0, 0.0], Condition::Always)
        .size_constraints([2.0, fixed_h], [f32::MAX, fixed_h])
        .size([300.0, 10f32], Condition::FirstUseEver)
        .flags(WindowFlags::NO_MOVE | WindowFlags::NO_COLLAPSE)
        .build(|| {
            let window_ui = &ui;
            let scene = &mut scene.lock().unwrap();

            ui.text("Infos:");
            ui.text(format_args!("fps: {}", fps.to_string()).to_string());

            let mut nb_vertices = 0;
            scene.shapes.iter().for_each(|x1| {
                nb_vertices += x1.mesh().mesh_data.vertices.len();
            });

            ui.text(format_args!("nb vertices: {}", nb_vertices.to_string()).to_string());

            if ui.button("Random Pos") {
                scene.shapes.iter_mut().for_each(|x| {
                    x.transform.set_position([
                        random::<f32>() * 5.0,
                        random::<f32>() * 5.0,
                        random::<f32>() * 5.0,
                    ]);
                });
            }

            if ui.button("New cube") {
                if let Some(shape) = scene.shapes.get(2).cloned() {
                    scene.add_shape(shape);
                }
            }

            if ui.button("Delete All") {
                while scene.shapes.len() > 0 {
                    scene.remove_shape(0);
                }
            }

            ui.separator();
            ui.text("Object list:");

            let _ = ui.push_id("camera");
            ui.text(scene.camera.get_object_name());
            scene.camera.get_object_ui(*window_ui);
            ui.separator();

            for (i, shape) in scene.shapes.iter_mut().enumerate() {
                let _id = ui.push_id_usize(i);
                ui.text(shape.get_object_name());
                shape.get_object_ui(*window_ui);
                ui.separator();
            }

            // lights
            let _ = ui.push_id(scene.lighting.dir_light.get_object_name());
            ui.text(scene.lighting.dir_light.get_object_name());
            scene.lighting.dir_light.get_object_ui(ui);
        });
}
