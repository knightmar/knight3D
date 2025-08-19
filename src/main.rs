mod scene;
mod shader;
mod texture;

use crate::scene::shape::Shape;
use crate::scene::Scene;
use gl;
use glfw::Key::Escape;
use glfw::{Context, Window};
use std::process::exit;
use std::ptr::null;

pub static mut TIME: f64 = 0.0;

/// In main, the glfw context and the window are created, gl is loaded, a shape and its indices is declared, and then the main loop starts
fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, _) = glfw
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

    // Rust
    let vertices: [([f32; 3], [f32; 3], [f32; 2]); 36] = [
        // Face avant
        ([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]), // bas gauche
        ([1.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]), // bas droit
        ([1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]), // haut droit
        ([1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]), // haut droit
        ([0.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]), // haut gauche
        ([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]), // bas gauche
        // Face arrière
        ([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 0.0]),
        ([0.0, 1.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0]),
        ([1.0, 1.0, 0.0], [0.0, 0.0, -1.0], [1.0, 1.0]),
        ([1.0, 1.0, 0.0], [0.0, 0.0, -1.0], [1.0, 1.0]),
        ([1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0]),
        ([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 0.0]),
        // Face gauche
        ([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
        ([0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 0.0]),
        ([0.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
        ([0.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
        ([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0]),
        ([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
        // Face droite
        ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
        ([1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0]),
        ([1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
        ([1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
        ([1.0, 0.0, 1.0], [1.0, 0.0, 0.0], [1.0, 0.0]),
        ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
        // Face du haut
        ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
        // Face du bas
        ([0.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0]),
        ([1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 0.0]),
        ([1.0, 0.0, 1.0], [0.0, -1.0, 0.0], [1.0, 1.0]),
        ([1.0, 0.0, 1.0], [0.0, -1.0, 0.0], [1.0, 1.0]),
        ([0.0, 0.0, 1.0], [0.0, -1.0, 0.0], [0.0, 1.0]),
        ([0.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0]),
    ];

    let mut shape = Shape::new(&vertices, None, "textures/dummy.png");
    shape.init_shaders("vertex_shader", "fragment_shader");

    let mut scene = Scene::new();
    scene.add_shape(shape);
    scene.camera.transform.translate([0.0, 1.0, 3.0]);
    scene.camera.transform.rotate([0.0, 1.0, 0.0], 15.0);

    let mut last_time = glfw.get_time();
    let mut frames = 0;

    while !window.should_close() {
        glfw.poll_events();
        process_input(&window);

        unsafe {
            TIME = glfw.get_time();

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            scene.render();
        }

        window.swap_buffers();

        frames += 1;
        let current_time = glfw.get_time();
        if current_time - last_time >= 1.0 {
            println!("FPS: {}", frames);
            frames = 0;
            last_time = current_time;
        }
    }
}

fn process_input(window: &Window) {
    if window.get_key(Escape) == glfw::Action::Press {
        exit(0);
    }
}
