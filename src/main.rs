mod shader;
mod shape;
mod texture;

use crate::shape::{Shape, UniformValue};
use gl;
use gl::types::GLsizei;
use glfw::Key::Escape;
use glfw::{Context, Window};
use std::process::exit;
use std::ptr::null;

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
    }

    window.set_framebuffer_size_callback(framebuffer_size_callback);

    let vertices: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
        ([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
        ([0.0, 1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 1.0]),
        ([1.0, 1.0, 0.0], [0.0, 0.0, 1.0], [1.0, 1.0]),
        ([1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [1.0, 0.0]),
    ];

    let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];

    let mut shape = Shape::new(&vertices, &indices, "textures/dummy.png");
    shape.init_shaders("vertex_shader", "fragment_shader");

    let mut tranform = nalgebra_glm::identity::<f32,4>();
    tranform = nalgebra_glm::rotate(&tranform, f32::to_radians(45.0), &nalgebra_glm::vec3(0.0, 1.0, 1.0));
    tranform = nalgebra_glm::scale(&tranform, &nalgebra_glm::vec3(2.0, 2.0, 2.0));

    while !window.should_close() {
        process_input(&window);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shape.render();
            shape.set_uniform("time", UniformValue::Float(glfw.get_time() as f32));

            shape.set_uniform("transform", UniformValue::Matrix4fv(tranform));
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
