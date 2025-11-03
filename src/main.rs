extern crate core;
use crate::renderer::Renderer;

mod renderer;
mod scene;
mod shader;
mod shape;
mod texture;
mod ui;

use crate::renderer::color_renderer::ColorRenderer;
use crate::scene::Scene;
use crate::shape::Shape;
use crate::ui::Ui;
use gl;
use glfw::Key::Escape;
use glfw::{Context, Window};
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

    let pos: [[f32; 3]; 24] = [
        // Face avant (-z)
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
        // Face arrière (+z)
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
        // Face gauche (-x)
        [-0.5, 0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        // Face droite (+x)
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        // Face bas (-y)
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        // Face haut (+y)
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
    ];

    let color = [
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];

    let indices: [u32; 36] = [
        0, 1, 2, 2, 3, 0, // avant
        4, 5, 6, 6, 7, 4, // arrière
        8, 9, 10, 10, 11, 8, // gauche
        12, 13, 14, 14, 15, 12, // droite
        16, 17, 18, 18, 19, 16, // bas
        20, 21, 22, 22, 23, 20, // haut
    ];

    let pos1: [[f32; 3]; 16] = [
        // Base (-y)
        [-0.5, -0.5, 0.5],  // 0
        [0.5, -0.5, 0.5],   // 1
        [0.5, -0.5, -0.5],  // 2
        [-0.5, -0.5, -0.5], // 3
        // Face avant (-z)
        [-0.5, -0.5, -0.5], // 4
        [0.5, -0.5, -0.5],  // 5
        [0.0, 0.5, 0.0],    // 6
        // Face arrière (+z)
        [0.5, -0.5, 0.5],  // 7
        [-0.5, -0.5, 0.5], // 8
        [0.0, 0.5, 0.0],   // 9
        // Face gauche (-x)
        [-0.5, -0.5, 0.5],  // 10
        [-0.5, -0.5, -0.5], // 11
        [0.0, 0.5, 0.0],    // 12
        // Face droite (+x)
        [0.5, -0.5, -0.5], // 13
        [0.5, -0.5, 0.5],  // 14
        [0.0, 0.5, 0.0],   // 15
    ];

    let color1: [[f32; 3]; 16] = [
        // Base (vert)
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        // Face avant (bleu)
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        // Face arrière (cyan)
        [0.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
        [0.0, 1.0, 1.0],
        // Face gauche (rouge)
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        // Face droite (magenta)
        [1.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
    ];

    let indices1: [u32; 18] = [
        // Base
        0, 1, 2, 2, 3, 0, // Face avant
        4, 5, 6, // Face arrière
        7, 8, 9, // Face gauche
        10, 11, 12, // Face droite
        13, 14, 15,
    ];


    fn generate_sphere(stacks: u32, sectors: u32) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>) {
        let mut pos = Vec::new();
        let mut color = Vec::new();
        let mut indices = Vec::new();
        let radius = 0.5;

        let stack_step = std::f32::consts::PI / stacks as f32;
        let sector_step = 2.0 * std::f32::consts::PI / sectors as f32;

        for i in 0..=stacks {
            let stack_angle = std::f32::consts::PI / 2.0 - (i as f32 * stack_step);
            let xy = radius * stack_angle.cos();
            let z = radius * stack_angle.sin();

            for j in 0..=sectors {
                let sector_angle = j as f32 * sector_step;
                let x = xy * sector_angle.cos();
                let y = xy * sector_angle.sin();
                pos.push([x, y, z]);
                // Couleur blanche pour toute la sphère
                color.push([1.0, 1.0, 1.0]);
            }
        }

        for i in 0..stacks {
            let mut k1 = i * (sectors + 1);
            let mut k2 = k1 + sectors + 1;

            for _j in 0..sectors {
                if i != 0 {
                    indices.push(k1);
                    indices.push(k2);
                    indices.push(k1 + 1);
                }

                if i != (stacks - 1) {
                    indices.push(k1 + 1);
                    indices.push(k2);
                    indices.push(k2 + 1);
                }
                k1 += 1;
                k2 += 1;
            }
        }

        (pos, color, indices)
    }


    let (post, colort, indicest) = generate_sphere(32, 32);

    let mut sphere_shape = Shape::new(
        "Sphere".to_string(),
        post.into_boxed_slice(),
        colort.into_boxed_slice(),
        Box::new([]), // Pas de coordonnées de texture pour l'instant
        Some(indicest),
        "./textures/dummy.png",
    );

    let mut shape = Shape::new(
        "Cube".to_string(),
        Box::from(pos),
        Box::from(color),
        Box::new([]),
        Some(Vec::from(&indices)),
        "./textures/dummy.png",
    );
    let mut shape1 = Shape::new(
        "triangle".to_string(),
        Box::from(pos1),
        Box::from(color1),
        Box::new([]),
        Some(Vec::from(&indices1)),
        "./textures/dummy.png",
    );

    // let mut shape2 =
    //     Shape::from_obj_file("cube".to_string(), "./obj/CUBE.obj", "./textures/dummy.png").unwrap();

    let mut scene = Arc::new(Mutex::new(Scene::new()));

    let mut ui = Ui::init(glfw, &mut window, events, scene.clone());

    // scene.lock().unwrap().add_shape(shape);
    scene.lock().unwrap().add_shape(shape1);
    scene.lock().unwrap().add_shape(sphere_shape);


    // scene.lock().unwrap().add_shape(shape2.clone());
    // scene.lock().unwrap().add_shape(shape.clone());

    let mut renderer = ColorRenderer::init("vertex_shader", "fragment_shader", scene.clone());
    renderer.init_buffers();

    renderer.get_obj_list().iter().for_each(|x2| {
        println!("{}", x2.name());
    });

    scene
        .lock()
        .unwrap()
        .camera
        .transform
        .translate([-5.0, 3.0, 20.0]);
    scene
        .lock()
        .unwrap()
        .camera
        .transform
        .rotate([0.0, -40.0, 0.0], 15.0);

    while (&ui.ui_data.window.should_close()).not() {
        scene.lock().unwrap().shapes.iter_mut().for_each(|x| {
            x.transform.rotate([90.0, 10.0, 10.0], 3.0);
        });

        ui.ui_data.glfw.poll_events();

        ui.process_inputs();
        process_input(&ui.ui_data.window);

        unsafe {
            TIME = ui.ui_data.glfw.get_time();
        }

        unsafe {
            let (fb_w, fb_h) = &ui.ui_data.window.get_framebuffer_size();
            gl::Viewport(0, 0, *fb_w, *fb_h);
            gl::ClearColor(0.3, 0.7, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        renderer.render();
        ui.render();
    }
}

fn process_input(window: &Window) {
    if window.get_key(Escape) == glfw::Action::Press {
        exit(0);
    }
}
