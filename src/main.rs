mod scene;
mod shader;
mod texture;
mod ui;
mod utils;
mod objects;

use crate::scene::Scene;
use objects::shape::Shape;
use crate::ui::Ui;
use gl;
use glfw::Key::Escape;
use glfw::{Context, Window};
use std::ops::Not;
use std::process::exit;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use crate::objects::Renderable;

pub static mut TIME: f64 = 0.0;

fn main() {
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    }
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(800, 600, "Hello World", glfw::WindowMode::Windowed)
        .unwrap();
    glfw.make_context_current(Some(&window));

    gl::load_with(|e| {
        glfw.get_proc_address_raw(e)
            .map_or(null(), |f| f as *const _)
    });
    unsafe {
        gl::Viewport(0, 0, 800, 600);
        gl::Enable(gl::DEPTH_TEST);

    }

    let vertices: [([f32; 3], [f32; 3], [f32; 2], [f32; 3]); 24] = [
        // --- Back Face (Z-) ---
        ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0], [0.0, 0.0, -1.0]),
        ([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0], [0.0, 0.0, -1.0]),
        ([1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0], [0.0, 0.0, -1.0]),
        ([0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0], [0.0, 0.0, -1.0]),

        // --- Front Face (Z+) ---
        ([0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 0.0], [0.0, 0.0, 1.0]),
        ([1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [1.0, 0.0], [0.0, 0.0, 1.0]),
        ([1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0], [0.0, 0.0, 1.0]),
        ([0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0], [0.0, 0.0, 1.0]),

        // --- Left Face (X-) ---
        ([0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 0.0], [-1.0, 0.0, 0.0]),
        ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0], [-1.0, 0.0, 0.0]),
        ([0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0], [-1.0, 0.0, 0.0]),
        ([0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0], [-1.0, 0.0, 0.0]),

        // --- Right Face (X+) ---
        ([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0], [1.0, 0.0, 0.0]),
        ([1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [1.0, 0.0], [1.0, 0.0, 0.0]),
        ([1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0], [1.0, 0.0, 0.0]),
        ([1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0], [1.0, 0.0, 0.0]),

        // --- Top Face (Y+) ---
        ([0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0], [0.0, 1.0, 0.0]),
        ([1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0], [0.0, 1.0, 0.0]),
        ([1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0], [0.0, 1.0, 0.0]),
        ([0.0, 1.0, 1.0], [1.0, 1.0, 1.0], [0.0, 1.0], [0.0, 1.0, 0.0]),

        // --- Bottom Face (Y-) ---
        ([0.0, 0.0, 1.0], [1.0, 1.0, 1.0], [0.0, 0.0], [0.0, -1.0, 0.0]),
        ([1.0, 0.0, 1.0], [1.0, 1.0, 1.0], [1.0, 0.0], [0.0, -1.0, 0.0]),
        ([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0], [0.0, -1.0, 0.0]),
        ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0], [0.0, -1.0, 0.0]),
    ];

    let vertices2: [([f32; 3], [f32; 3], [f32; 2], [f32; 3]); 16] = [
        // --- SIDE 1 (front: Y is decreasing) ---
        // Normal calculation: Cross product of (1,0,0) and (0.5, 0.5, 1) -> (0, -1, 0.5) normalized
        ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0], [0.0, -0.894, 0.447]), // base front-left
        ([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0], [0.0, -0.894, 0.447]), // base front-right
        ([0.5, 0.5, 1.0], [1.0, 1.0, 1.0], [0.5, 1.0], [0.0, -0.894, 0.447]), // apex

        // --- SIDE 2 (right: X is increasing) ---
        // Normal calculation: (1, 0, 0.5) normalized
        ([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0], [0.894, 0.0, 0.447]),
        ([1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0], [0.894, 0.0, 0.447]),
        ([0.5, 0.5, 1.0], [1.0, 1.0, 1.0], [0.5, 1.0], [0.894, 0.0, 0.447]),

        // --- SIDE 3 (back: Y is increasing) ---
        // Normal calculation: (0, 1, 0.5) normalized
        ([1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0], [0.0, 0.894, 0.447]),
        ([0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0], [0.0, 0.894, 0.447]),
        ([0.5, 0.5, 1.0], [1.0, 1.0, 1.0], [0.5, 1.0], [0.0, 0.894, 0.447]),

        // --- SIDE 4 (left: X is decreasing) ---
        // Normal calculation: (-1, 0, 0.5) normalized
        ([0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0], [-0.894, 0.0, 0.447]),
        ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0], [-0.894, 0.0, 0.447]),
        ([0.5, 0.5, 1.0], [1.0, 1.0, 1.0], [0.5, 1.0], [-0.894, 0.0, 0.447]),

        // --- BASE (square) ---
        // Normal is pointing straight down
        ([0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.0, 0.0], [0.0, 0.0, -1.0]),
        ([1.0, 0.0, 0.0], [1.0, 1.0, 1.0], [1.0, 0.0], [0.0, 0.0, -1.0]),
        ([1.0, 1.0, 0.0], [1.0, 1.0, 1.0], [1.0, 1.0], [0.0, 0.0, -1.0]),
        ([0.0, 1.0, 0.0], [1.0, 1.0, 1.0], [0.0, 1.0], [0.0, 0.0, -1.0]),
    ];

    let indices: [u32; 36] = [
        // Avant
        0, 1, 2, 0, 2, 3, // Arrière
        4, 6, 5, 4, 7, 6, // Gauche
        8, 9, 10, 8, 10, 11, // Droite
        12, 13, 14, 12, 14, 15, // Haut
        16, 17, 18, 16, 18, 19, // Bas
        20, 21, 22, 20, 22, 23,
    ];

    let indices2: [u32; 18] = [
        // Side 1
        0, 1, 2, // Side 2
        3, 4, 5, // Side 3
        6, 7, 8, // Side 4
        9, 10, 11, // Base (two triangles)
        12, 13, 14, 12, 14, 15,
    ];

    let mut shape2 = Shape::new(
        "Pyramid".into(),
        Box::from(vertices2),
        Some(indices2.to_vec()),
        "./textures/debug.png",
    );
    shape2.init_shaders("vertex_shader", "fragment_shader");

    let mut shape = Shape::new(
        "Cube".into(),
        Box::from(vertices),
        Some(indices.to_vec()),
        "./textures/dummy.png",
    );
    shape.init_shaders("vertex_shader", "fragment_shader");

    let mut obj = Shape::from_obj_file(
        "obj".into(),
        "./obj/car.obj",
        "./textures/car.jpg",
    )
    .unwrap();
    obj.init_shaders("vertex_shader", "fragment_shader");
    obj.transform.set_scale([0.01, 0.01, 0.01]);
    obj.transform.rotate([1.0, 0.0, 0.0], -0.0);

    let scene = Arc::new(Mutex::new(Scene::new()));

    let mut ui = Ui::init(glfw, &mut window, events, scene.clone());

    scene.lock().unwrap().add_shape(shape.clone());
    scene.lock().unwrap().add_shape(shape2.clone());
    scene.lock().unwrap().add_shape(obj.clone());
    scene
        .lock()
        .unwrap()
        .camera
        .transform
        .translate([0.0, 1.0, 10.0]);
    scene
        .lock()
        .unwrap()
        .camera
        .transform
        .rotate([0.0, -40.0, 0.0], 15.0);

    while (&ui.ui_data.window.should_close()).not() {
        ui.ui_data.glfw.poll_events();

        ui.process_inputs();
        process_input(&ui.ui_data.window);

        unsafe {
            TIME = ui.ui_data.glfw.get_time();
        }

        unsafe {
            let (fb_w, fb_h) = &ui.ui_data.window.get_framebuffer_size();
            gl::Viewport(0, 0, *fb_w, *fb_h);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        scene.lock().unwrap().render();
        ui.render();
    }
}

fn process_input(window: &Window) {
    if window.get_key(Escape) == glfw::Action::Press {
        exit(0);
    }
}
