mod left_panel;

use crate::scene::Scene;
use crate::ui::left_panel::left_panel_ui;
use glfw::{Action, Context, Glfw, GlfwReceiver, MouseButton, PWindow, WindowEvent};
use imgui::Context as ImGuiContext;
use imgui_opengl_renderer::{Renderer as ImGuiRenderer, Renderer};
use std::ops::Index;
use std::ptr::null;
use std::sync::{Arc, Mutex};

pub struct Ui<'a> {
    pub ui_data: UIData<'a>,
}

pub struct UIData<'a> {
    pub glfw: Glfw,
    context: ImGuiContext,
    pub window: &'a mut PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    scene: Arc<Mutex<Scene>>,
    renderer: Renderer,
    last_imgui_time: f64,
    last_fps_time: f64,
    frames: u64,
    fps: u64,
}

fn translate_glfw_key(key: glfw::Key) -> Option<imgui::Key> {
    use glfw::Key::*;
    use imgui::Key;

    Some(match key {
        Tab => Key::Tab,
        Left => Key::LeftArrow,
        Right => Key::RightArrow,
        Up => Key::UpArrow,
        Down => Key::DownArrow,
        PageUp => Key::PageUp,
        PageDown => Key::PageDown,
        Home => Key::Home,
        End => Key::End,
        Insert => Key::Insert,
        Delete => Key::Delete,
        Backspace => Key::Backspace,
        Space => Key::Space,
        Enter => Key::Enter,
        Escape => Key::Escape,
        KpEnter => Key::KeyPadEnter,
        A => Key::A,
        C => Key::C,
        V => Key::V,
        X => Key::X,
        Y => Key::Y,
        Z => Key::Z,

        _ => return None,
    })
}

impl<'a> Ui<'a> {
    pub fn init(
        glfw: Glfw,
        window: &'a mut PWindow,
        events: GlfwReceiver<(f64, WindowEvent)>,
        scene: Arc<Mutex<Scene>>,
    ) -> Self {
        window.set_key_polling(true);
        window.set_char_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_pos_polling(true);

        let mut imgui = ImGuiContext::create();
        imgui.set_ini_filename(None);

        // translation from imgui to glfw keys
        {
            use imgui::{ConfigFlags, Key};
            let io = imgui.io_mut();
            io[Key::Tab] = glfw::Key::Tab as u32;
            io[Key::LeftArrow] = glfw::Key::Left as u32;
            io[Key::RightArrow] = glfw::Key::Right as u32;
            io[Key::UpArrow] = glfw::Key::Up as u32;
            io[Key::DownArrow] = glfw::Key::Down as u32;
            io[Key::PageUp] = glfw::Key::PageUp as u32;
            io[Key::PageDown] = glfw::Key::PageDown as u32;
            io[Key::Home] = glfw::Key::Home as u32;
            io[Key::End] = glfw::Key::End as u32;
            io[Key::Insert] = glfw::Key::Insert as u32;
            io[Key::Delete] = glfw::Key::Delete as u32;
            io[Key::Backspace] = glfw::Key::Backspace as u32;
            io[Key::Space] = glfw::Key::Space as u32;
            io[Key::Enter] = glfw::Key::Enter as u32;
            io[Key::Escape] = glfw::Key::Escape as u32;
            io[Key::KeyPadEnter] = glfw::Key::KpEnter as u32;
            io[Key::A] = glfw::Key::A as u32;
            io[Key::C] = glfw::Key::C as u32;
            io[Key::V] = glfw::Key::V as u32;
            io[Key::X] = glfw::Key::X as u32;
            io[Key::Y] = glfw::Key::Y as u32;
            io[Key::Z] = glfw::Key::Z as u32;

            io.config_flags |= ConfigFlags::NAV_ENABLE_KEYBOARD;
        }

        let imgui_renderer = ImGuiRenderer::new(&mut imgui, |s| {
            glfw.get_proc_address_raw(s).map_or(null(), |p| p as _)
        });

        window.set_framebuffer_size_polling(true);
        window.set_content_scale_polling(true);

        let last_imgui_time = glfw.get_time();

        Ui {
            ui_data: UIData {
                glfw,
                context: imgui,
                renderer: imgui_renderer,
                window,
                events,
                scene,
                last_imgui_time,
                last_fps_time: last_imgui_time,
                frames: 0,
                fps: 0,
            },
        }
    }

    pub fn process_inputs(&mut self) {
        let mut scroll_x: f32 = 0.0;
        let mut scroll_y: f32 = 0.0;
        let mut typed_chars: Vec<char> = Vec::new();

        for (_, event) in glfw::flush_messages(&self.ui_data.events) {
            match event {
                WindowEvent::Scroll(x, y) => {
                    scroll_x += x as f32;
                    scroll_y += y as f32;
                }
                WindowEvent::Char(c) => {
                    typed_chars.push(c);
                }
                WindowEvent::Key(key, _, action, mods) => {
                    let io = self.ui_data.context.io_mut();
                    let pressed = action != Action::Release;

                    if let Some(imgui_key) = translate_glfw_key(key) {
                        let idx = *io.index(imgui_key) as usize; // idx comes from key_map
                        if idx < io.keys_down.len() {
                            io.keys_down[idx] = pressed;
                        }
                    }

                    io.key_shift = mods.contains(glfw::Modifiers::Shift);
                    io.key_ctrl = mods.contains(glfw::Modifiers::Control);
                    io.key_alt = mods.contains(glfw::Modifiers::Alt);
                    io.key_super = mods.contains(glfw::Modifiers::Super);
                }
                WindowEvent::FramebufferSize(w, h) => unsafe {
                    gl::Viewport(0, 0, w, h);
                    self.ui_data
                        .scene
                        .lock()
                        .unwrap()
                        .camera
                        .set_aspect(w as f32, h as f32);
                },
                WindowEvent::ContentScale(x, y) => unsafe {
                    let (fb_w, fb_h) = self.ui_data.window.get_framebuffer_size();
                    gl::Viewport(0, 0, fb_w, fb_h);
                },
                _ => {}
            }
        }

        let now = self.ui_data.glfw.get_time();
        let delta = (now - self.ui_data.last_imgui_time) as f32;
        self.ui_data.last_imgui_time = now;

        let io = self.ui_data.context.io_mut();
        io.delta_time = if delta > 0.0 { delta } else { 1.0 / 60.0 };

        let (win_w, win_h) = self.ui_data.window.get_size();
        let (fb_w, fb_h) = self.ui_data.window.get_framebuffer_size();
        io.display_size = [win_w as f32, win_h as f32];
        io.display_framebuffer_scale = [
            fb_w as f32 / win_w.max(1) as f32,
            fb_h as f32 / win_h.max(1) as f32,
        ];

        let (mx, my) = self.ui_data.window.get_cursor_pos();
        io.mouse_pos = [mx as f32, my as f32];
        io.mouse_down[0] =
            self.ui_data.window.get_mouse_button(MouseButton::Button1) == Action::Press;
        io.mouse_down[2] =
            self.ui_data.window.get_mouse_button(MouseButton::Button3) == Action::Press;
        io.mouse_down[1] =
            self.ui_data.window.get_mouse_button(MouseButton::Button2) == Action::Press;

        io.mouse_wheel_h = scroll_x;
        io.mouse_wheel = scroll_y;

        for c in typed_chars {
            io.add_input_character(c);
        }

        {
            use glfw::{Action, Key};
            use nalgebra as na;

            let io = self.ui_data.context.io();
            let speed: f32 = 3.0 * io.delta_time;

            let win = &self.ui_data.window;
            let forward_down = win.get_key(Key::W) == Action::Press;
            let back_down = win.get_key(Key::S) == Action::Press;
            let left_down = win.get_key(Key::A) == Action::Press;
            let right_down = win.get_key(Key::D) == Action::Press;
            let up_down = win.get_key(Key::Space) == Action::Press;
            let down_down = win.get_key(Key::LeftShift) == Action::Press;

            if forward_down || back_down || left_down || right_down || up_down || down_down {
                let mut scene = self.ui_data.scene.lock().unwrap();
                let cam = &mut scene.camera;

                let rot = na::UnitQuaternion::from_quaternion(cam.transform.rotation);
                let forward: na::Vector3<f32> = rot * na::Vector3::new(0.0, 0.0, -1.0);
                let right: na::Vector3<f32> = rot * na::Vector3::new(1.0, 0.0, 0.0);
                let up: na::Vector3<f32> = na::Vector3::new(0.0, 1.0, 0.0); // world up

                let mut delta = na::Vector3::zeros();
                if forward_down {
                    delta += forward;
                }
                if back_down {
                    delta -= forward;
                }
                if right_down {
                    delta += right;
                }
                if left_down {
                    delta -= right;
                }
                if up_down {
                    delta += up;
                }
                if down_down {
                    delta -= up;
                }

                let n = delta.norm();
                if n > 0.0 {
                    let step = (delta / n) * speed;
                    cam.transform.translate([step.x, step.y, step.z]);
                }
            }
        }
        
        // Camera look
        {
            use glfw::Action;
            use nalgebra as na;

            // Only when RMB is held
            let rmb_down = self
                .ui_data
                .window
                .get_mouse_button(glfw::MouseButton::Button2)
                == Action::Press;

            static mut LAST_MOUSE: Option<(f32, f32)> = None;

            let io_ro = self.ui_data.context.io();
            let sens_deg_per_px: f32 = 0.12;
            let sens = sens_deg_per_px;

            let (mx, my) = {
                let io = self.ui_data.context.io();
                (io.mouse_pos[0], io.mouse_pos[1])
            };

            if rmb_down {
                let (dx, dy) = unsafe {
                    match LAST_MOUSE {
                        Some((lx, ly)) => (mx - lx, my - ly),
                        None => (0.0, 0.0),
                    }
                };

                unsafe {
                    LAST_MOUSE = Some((mx, my));
                }

                if dx != 0.0 || dy != 0.0 {
                    let mut scene = self.ui_data.scene.lock().unwrap();
                    let cam = &mut scene.camera;
                    let yaw_deg = -dx * sens;
                    let pitch_deg = -dy * sens;
                    let rot_u = na::UnitQuaternion::from_quaternion(cam.transform.rotation);

                    let yaw_q = na::UnitQuaternion::from_axis_angle(
                        &na::Vector3::y_axis(),
                        yaw_deg.to_radians(),
                    );
                    let pitch_q = na::UnitQuaternion::from_axis_angle(
                        &na::Vector3::x_axis(),
                        pitch_deg.to_radians(),
                    );

                    let new_rot = yaw_q * rot_u * pitch_q;
                    cam.transform.rotation = new_rot.into_inner();
                }
            } else {
                unsafe {
                    LAST_MOUSE = None;
                }
            }
        }
    }

    pub fn render(&mut self) {
        let now = self.ui_data.glfw.get_time();
        let delta = (now - self.ui_data.last_fps_time) as f32;
        self.ui_data.frames += 1;
        if delta >= 1.0 {
            self.ui_data.fps = self.ui_data.frames;
            self.ui_data.frames = 0;
            self.ui_data.last_fps_time = now;
            println!("{}", self.ui_data.fps);
        }

        let fixed_h = self.ui_data.context.io().display_size[1];

        let scene = self.ui_data.scene.clone();

        let fps_u32: u32 = self.ui_data.fps as u32;
        let ui = self.ui_data.context.frame();
        left_panel_ui(ui, &scene, fps_u32, fixed_h);

        self.ui_data.renderer.render(&mut self.ui_data.context);
        self.ui_data.window.swap_buffers();
    }
}

pub trait Inspectable {
    fn get_object_ui(&mut self, ui: &imgui::Ui);
    fn get_object_name(&self) -> String;
}
