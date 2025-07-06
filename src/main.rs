extern crate glad_gl;

use glad_gl::gl;
use glad_gl::gl::{GLsizei, GLsizeiptr, GLuint};
use glfw::Key::Escape;
use glfw::{Context, Window};
use std::process::exit;
use std::ptr::null;

fn main() {
    let mut glfw1 = glfw::init(glfw::fail_on_errors).unwrap();
    glfw1.window_hint(glfw::WindowHint::ContextVersion(4, 6));
    glfw1.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let mut window = glfw1
        .create_window(800, 600, "Hello World", glfw::WindowMode::Windowed)
        .unwrap()
        .0;
    glfw1.make_context_current(Some(&window));

    gl::load(|e| glfw1.get_proc_address_raw(e) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, 800, 600);
    }

    window.set_framebuffer_size_callback(framebuffer_size_callback);

    let shape: [[f32; 3]; 3] = [
        [-0.5f32, -0.5f32, 0.0f32],
        [0.5f32, -0.5f32, 0.0f32],
        [0.0f32, 0.5f32, 0.0f32],
    ];

    let mut VBO: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut VBO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    }

    let vertex_shader_source: &str = r##"#version 460 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}"##;

    let mut vertex_shader: u32 = 0;
    unsafe {
        vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        let sources = [vertex_shader_source.as_ptr() as *const i8];
        gl::ShaderSource(vertex_shader, 1, sources.as_ptr(), null());
        gl::CompileShader(vertex_shader);
    };

    let fragment_shader_source: &str = r##"#version 460 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} "##;

    let mut fragment_shader: u32 = 0;
    unsafe {
        fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let sources = [fragment_shader_source.as_ptr() as *const i8];
        gl::ShaderSource(fragment_shader, 1, sources.as_ptr(), null());
        gl::CompileShader(fragment_shader);
    };

    let mut shader_program: u32 = 0;
    unsafe {
        shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        gl::UseProgram(shader_program);
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * size_of::<f32>()) as GLsizei,
            0 as *mut std::os::raw::c_void,
        );
        gl::EnableVertexAttribArray(0);
    }

    let mut VAO: u32 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut VAO);
        gl::BindVertexArray(VAO);
        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of::<[[f32; 3]; 3]>() as GLsizeiptr,
            shape.as_ptr() as *const std::os::raw::c_void,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            (3 * size_of::<f32>()) as GLsizei,
            0 as *mut std::os::raw::c_void,
        );
        gl::EnableVertexAttribArray(0);
    }

    while !window.should_close() {
        process_input(&window);

        unsafe {
            gl::UseProgram(shader_program);
            gl::ClearColor(0.2f32, 0.3f32, 0.3f32, 1.0f32);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(VAO);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

        }

        window.swap_buffers();
        glfw1.poll_events();
    }

    println!("Hello, world!");
}

fn framebuffer_size_callback(window: &mut Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width as GLsizei, height as GLsizei);
    }
}

fn process_input(window: &Window) {
    if window.get_key(Escape) == glfw::Action::Press {
        exit(0);
    }
}
