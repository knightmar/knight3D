mod objects;
mod scene;
mod shader;
mod texture;
mod ui;
mod utils;

use crate::objects::light::point_light::PointLight;
use crate::objects::material::Material;
use crate::objects::{Renderable, Transform};
use crate::scene::Scene;
use crate::texture::Texture;
use crate::ui::Ui;
use gl;
use glfw::Key::Escape;
use glfw::{Context, Window};
use objects::shape::Shape;
use std::ops::Not;
use std::process::exit;
use std::ptr::null;
use std::sync::{Arc, Mutex};

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
    
    
    
    let scene = Scene::load_scene_from_gltf("./level/scene.glb");
    

    let mut obj = Shape::from_obj_file(
        "obj".into(),
        "./obj/car.obj",
        Material {
            ambient: Texture::new("./textures/car.jpg").unwrap(),
            specular: Some(Texture::new("./textures/car_specular_test.jpeg").unwrap()),
            shininess: 32.0,
        },
    )
    .unwrap();
    obj.init_shaders("vertex_shader", "fragment_shader");
    // obj.transform.set_scale([0.01, 0.01, 0.01]);
    obj.transform.rotate([1.0, 0.0, 0.0], -0.0);

    let scene = Arc::new(Mutex::new(Scene::new()));

    let mut ui = Ui::init(glfw, &mut window, events, scene.clone());

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
    scene
        .lock()
        .unwrap()
        .lighting
        .point_lights
        .push(PointLight {
            name: "Pointlight".to_string(),
            transform: Transform {
                position: [-10.0, 5.0, 0.0],
                rotation: Default::default(),
                rotation_ui: [0.0, 0.0, 0.0],
                rotation_ui_editing: false,
                scale: [0.0, 0.0, 0.0],
            },
            ambient: [0.0, 0.0, 1.0],
            diffuse: [0.0, 0.0, 1.0],
            specular: [0.0, 0.0, 1.0],
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        });

    scene
        .lock()
        .unwrap()
        .lighting
        .point_lights
        .push(PointLight {
            name: "Pointlight".to_string(),
            transform: Transform {
                position: [10.0, 5.0, 0.0],
                rotation: Default::default(),
                rotation_ui: [0.0, 0.0, 0.0],
                rotation_ui_editing: false,
                scale: [0.0, 0.0, 0.0],
            },
            ambient: [1.0, 0.0, 0.0],
            diffuse: [1.0, 0.0, 0.0],
            specular: [1.0, 0.0, 0.0],
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        });
    scene
        .lock()
        .unwrap()
        .lighting
        .point_lights
        .push(PointLight {
            name: "Pointlight2".to_string(),
            transform: Transform::default(),
            ambient: [1.0, 1.0, 1.0],
            diffuse: [1.0, 1.0, 1.0],
            specular: [1.0, 1.0, 1.0],
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        });

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
