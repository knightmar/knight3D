mod shape;

use shape::shaders::Shader;
use glfw::Key::Escape;
use glfw::{Context, Window};
use std::mem::size_of;
use std::process::exit;
use std::ptr::null;
use gl::{FRAGMENT_SHADER, VERTEX_SHADER};
use gl::types::{GLsizei, GLsizeiptr, GLuint};
use gl;
use crate::shape::Shape;

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

    gl::load_with(|e| glfw.get_proc_address_raw(e).map_or(null(), |f| f as *const _));
    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    window.set_framebuffer_size_callback(framebuffer_size_callback);

    let vertices: [[f32; 3]; 4] = [
        [0.5,  0.5, 0.0],   // top right
        [0.5, -0.5, 0.0],   // bottom right
        [-0.5, -0.5, 0.0],  // bottom left
        [-0.5,  0.5, 0.0],  // top left
    ];

    let indices: [u32; 6] = [
        0, 1, 3,
        1, 2, 3
    ];

    let mut shape = Shape::new(&vertices, &indices);
    shape.init_shaders("vertex_shader", "fragment_shader");

    while !window.should_close() {
        process_input(&window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shape.render();
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn framebuffer_size_callback(_window: &mut Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
    }
}

fn process_input(window: &Window) {
    if window.get_key(Escape) == glfw::Action::Press {
        exit(0);
    }
}
