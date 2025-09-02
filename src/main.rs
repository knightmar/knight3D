mod scene;
mod shader;
mod texture;

use crate::scene::shape::Shape;
use crate::scene::Scene;
use gl;
use glfw::Key::Escape;
use glfw::{Action, Context, MouseButton, Window, WindowEvent};
use imgui::{Condition, Context as ImGuiContext, Window as ImGuiWindow};
use imgui_opengl_renderer::Renderer as ImGuiRenderer;
use std::process::exit;
use std::ptr::null;

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

    let mut imgui = ImGuiContext::create();
    imgui.set_ini_filename(None);

    let imgui_renderer = ImGuiRenderer::new(&mut imgui, |s| {
        glfw.get_proc_address_raw(s).map_or(null(), |p| p as _)
    });
    let mut last_imgui_time = glfw.get_time();

    // Remplace les tableaux `vertices` et `indices`
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
        0, 1, 2, 0, 2, 3,
        // Arrière
        4, 6, 5, 4, 7, 6,
        // Gauche
        8, 9, 10, 8, 10, 11,
        // Droite
        12, 13, 14, 12, 14, 15,
        // Haut
        16, 17, 18, 16, 18, 19,
        // Bas
        20, 21, 22, 20, 22, 23,
    ];

    let mut shape = Shape::new(&vertices, Some(&indices), "textures/dummy.png");
    shape.init_shaders("vertex_shader", "fragment_shader");

    let mut scene = Scene::new();
    scene.add_shape(shape);
    scene.camera.transform.translate([0.0, 1.0, 3.0]);
    scene.camera.transform.rotate([0.0, -40.0, 0.0], 15.0);

    let mut last_time = glfw.get_time();
    let mut frames = 0;

    let mut scroll_x: f32 = 0.0;
    let mut scroll_y: f32 = 0.0;
    let mut typed_chars: Vec<char> = Vec::new();

    while !window.should_close() {
        if let Some(mut s) = scene.shapes.get_mut(0) {
            s.transform.rotate([1.0,1.0,0.0], 1.0);
        }

        glfw.poll_events();
        scroll_x = 0.0;
        scroll_y = 0.0;
        typed_chars.clear();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Scroll(x, y) => {
                    scroll_x += x as f32;
                    scroll_y += y as f32;
                }
                WindowEvent::Char(c) => {
                    typed_chars.push(c);
                }
                _ => {}
            }
        }

        process_input(&window);

        unsafe {
            TIME = glfw.get_time();
        }
        let (fb_w, fb_h) = window.get_framebuffer_size();
        unsafe {
            gl::Viewport(0, 0, fb_w, fb_h);
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let now = glfw.get_time();
        let delta = (now - last_imgui_time) as f32;
        last_imgui_time = now;

        {
            let io = imgui.io_mut();
            let (win_w, win_h) = window.get_size();
            let (fb_w, fb_h) = window.get_framebuffer_size();
            io.delta_time = if delta > 0.0 { delta } else { 1.0 / 60.0 };
            io.display_size = [win_w as f32, win_h as f32];
            io.display_framebuffer_scale = [
                fb_w as f32 / win_w.max(1) as f32,
                fb_h as f32 / win_h.max(1) as f32,
            ];

            let (mx, my) = window.get_cursor_pos();
            io.mouse_pos = [mx as f32, my as f32];
            io.mouse_down[0] = window.get_mouse_button(MouseButton::Button1) == Action::Press;
            io.mouse_down[1] = window.get_mouse_button(MouseButton::Button2) == Action::Press;
            io.mouse_down[2] = window.get_mouse_button(MouseButton::Button3) == Action::Press;

            io.mouse_wheel_h = scroll_x;
            io.mouse_wheel = scroll_y;

            for c in typed_chars.drain(..) {
                io.add_input_character(c);
            }
        }

        let ui = imgui.frame();
        ImGuiWindow::new(&ui, "Prévisualisation FBO")
            .size([400.0, 300.0], Condition::FirstUseEver)
            .build(|| {
                if ui.button("test") {
                    println!("test");
                }
            });

        unsafe {
            gl::Viewport(0, 0, fb_w, fb_h);
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        scene.render();
        imgui_renderer.render(&mut imgui);

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
