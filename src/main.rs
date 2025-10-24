mod scene;
mod shader;
mod texture;
mod ui;
mod shape;

use rand::random;
use crate::scene::Scene;
use crate::ui::Ui;
use gl;
use glfw::Key::Escape;
use glfw::{Context, Window};
use std::ops::Not;
use std::process::exit;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use crate::shape::Shape;

pub static mut TIME: f64 = 0.0;

fn main() {

    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
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

    let vertices: [([f32; 3], [f32; 3], [f32; 2]); 24] = [
        ([0.0, 0.0, 0.0], [0.0, 0.0, -1.0], [0.0, 0.0]),
        ([1.0, 0.0, 0.0], [0.0, 0.0, -1.0], [1.0, 0.0]),
        ([1.0, 1.0, 0.0], [0.0, 0.0, -1.0], [1.0, 1.0]),
        ([0.0, 1.0, 0.0], [0.0, 0.0, -1.0], [0.0, 1.0]),
        ([0.0, 0.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
        ([1.0, 0.0, 1.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
        ([1.0, 1.0, 1.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
        ([0.0, 1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 1.0]),
        ([0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 0.0]),
        ([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0]),
        ([0.0, 1.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 1.0]),
        ([0.0, 1.0, 1.0], [-1.0, 0.0, 0.0], [0.0, 1.0]),
        ([1.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
        ([1.0, 0.0, 1.0], [1.0, 0.0, 0.0], [1.0, 0.0]),
        ([1.0, 1.0, 1.0], [1.0, 0.0, 0.0], [1.0, 1.0]),
        ([1.0, 1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0]),
        ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
        ([1.0, 1.0, 0.0], [0.0, 1.0, 0.0], [1.0, 0.0]),
        ([1.0, 1.0, 1.0], [0.0, 1.0, 0.0], [1.0, 1.0]),
        ([0.0, 1.0, 1.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([0.0, 0.0, 1.0], [0.0, -1.0, 0.0], [0.0, 0.0]),
        ([1.0, 0.0, 1.0], [0.0, -1.0, 0.0], [1.0, 0.0]),
        ([1.0, 0.0, 0.0], [0.0, -1.0, 0.0], [1.0, 1.0]),
        ([0.0, 0.0, 0.0], [0.0, -1.0, 0.0], [0.0, 1.0]),
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

    let mut shape = Shape::new("Cube", Box::from(vertices), Some(&indices), "./textures/dummy.png");
    shape.init_shaders("vertex_shader", "fragment_shader");

    // let mut shape2 = Shape::from_obj_file("cube", "./obj/CUBE.obj", "./textures/dummy.png").unwrap();
    // shape2.init_shaders("vertex_shader", "fragment_shader");

    let mut scene = Arc::new(Mutex::new(Scene::new()));

    let mut ui = Ui::init(glfw, &mut window, events, scene.clone());

    scene.lock().unwrap().add_shape(shape.clone());
    // scene.lock().unwrap().add_shape(shape2.clone());
    // scene.lock().unwrap().add_shape(shape.clone());
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
        scene.lock().unwrap().shapes.iter_mut().for_each(|mut x| {
            x.transform.rotate([random::<f32>()*10.0, random::<f32>() * 10.0,random::<f32>()*10.0], 0.1);
        });

        ui.ui_data.glfw.poll_events();



        ui.process_inputs();
        process_input(&ui.ui_data.window);

        unsafe {
            TIME = ui.ui_data.glfw.get_time();
        }
        let (fb_w, fb_h) = &ui.ui_data.window.get_framebuffer_size();

        unsafe {
            gl::Viewport(0, 0, *fb_w, *fb_h);
            gl::ClearColor(0.3, 0.7, 0.5, 1.0);
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
