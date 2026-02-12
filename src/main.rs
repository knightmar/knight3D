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
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::prelude::*;
use glutin_winit::{DisplayBuilder, GlWindow};
use objects::shape::Shape;
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub static mut TIME: f64 = 0.0;
pub static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

fn main() {
    // unsafe {
    //     std::env::set_var("GDK_BACKEND", "x11");
    // }
    START_TIME.set(Instant::now()).unwrap();
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let window_builder = WindowBuilder::new()
        .with_title("Knight3D")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

    let template = glutin::config::ConfigTemplateBuilder::new();
    let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

    let (window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs
                .reduce(|accum, config| {
                    if config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let window = window.expect("Failed to create window");
    let gl_display = gl_config.display();
    let context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
        .build(Some(window.raw_window_handle()));

    let not_current_gl_context = unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .expect("failed to create context")
    };

    let attrs = window.build_surface_attributes(Default::default());
    let gl_surface = unsafe {
        gl_display
            .create_window_surface(&gl_config, &attrs)
            .unwrap()
    };

    let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

    gl::load_with(|symbol| {
        let symbol = std::ffi::CString::new(symbol).unwrap();
        gl_display.get_proc_address(symbol.as_c_str()).cast()
    });

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

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
    obj.transform.rotate([1.0, 0.0, 0.0], -0.0);

    let scene = Arc::new(Mutex::new(Scene::new()));

    let mut ui = Ui::init(&window, scene.clone(), |symbol| {
        let symbol = std::ffi::CString::new(symbol).unwrap();
        gl_display.get_proc_address(symbol.as_c_str()).cast()
    });

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

    {
        let mut s = scene.lock().unwrap();
        s.lighting.point_lights.push(PointLight {
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
        s.lighting.point_lights.push(PointLight {
            name: "Pointlight2".to_string(),
            transform: Transform::default(),
            ambient: [1.0, 1.0, 1.0],
            diffuse: [1.0, 1.0, 1.0],
            specular: [1.0, 1.0, 1.0],
            constant: 1.0,
            linear: 0.09,
            quadratic: 0.032,
        });
    }

    event_loop
        .run(move |event, window_target| {
            window_target.set_control_flow(ControlFlow::Poll);
            ui.handle_event(&window, &event);

            match event {
                Event::NewEvents(_) => {
                    ui.update_delta_time();
                    unsafe {
                        TIME = START_TIME.get().unwrap().elapsed().as_secs_f64();
                    }
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        window_target.exit();
                    }
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            gl_surface.resize(
                                &gl_context,
                                NonZeroU32::new(size.width).unwrap(),
                                NonZeroU32::new(size.height).unwrap(),
                            );
                        }
                    }
                    WindowEvent::RedrawRequested => {
                        unsafe {
                            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
                            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
                        }

                        scene.lock().unwrap().render();

                        ui.prepare_frame(&window);
                        ui.render();

                        gl_surface.swap_buffers(&gl_context).unwrap();
                    }
                    _ => {}
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .expect("Failed to run event loop");
}
