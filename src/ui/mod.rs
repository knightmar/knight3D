mod left_panel;

use crate::scene::Scene;
use crate::ui::left_panel::left_panel_ui;
use imgui::Context as ImGuiContext;
use imgui_glow_renderer::{Renderer, SimpleTextureMap};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use winit::event::WindowEvent;
use winit::window::Window;

pub struct Ui {
    pub ui_data: UIData,
}

pub struct UIData {
    pub context: ImGuiContext,
    pub platform: WinitPlatform,
    pub scene: Arc<Mutex<Scene>>,
    pub renderer: Renderer,
    pub textures: SimpleTextureMap,
    pub glow_context: Arc<glow::Context>,
    last_imgui_time: Instant,
    last_fps_time: Instant,
    frames: u64,
    fps: u64,
}

impl Ui {
    pub fn init(
        window: &Window,
        scene: Arc<Mutex<Scene>>,
        mut gl_loader: impl FnMut(&str) -> *const std::ffi::c_void,
    ) -> Self {
        let mut imgui = ImGuiContext::create();
        imgui.set_ini_filename(None);

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), window, HiDpiMode::Default);

        let glow_context = unsafe { imgui_glow_renderer::glow::Context::from_loader_function(|s| gl_loader(s)) };
        let glow_context = Arc::new(glow_context);

        let mut textures = SimpleTextureMap::default();

        let imgui_renderer = Renderer::initialize(
            glow_context.as_ref(),
            &mut imgui,
            &mut textures,
            true,
        )
        .expect("Failed to initialize imgui renderer");

        let now = Instant::now();

        Ui {
            ui_data: UIData {
                context: imgui,
                platform,
                renderer: imgui_renderer,
                textures,
                glow_context,
                scene,
                last_imgui_time: now,
                last_fps_time: now,
                frames: 0,
                fps: 0,
            },
        }
    }

    pub fn update_delta_time(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.ui_data.last_imgui_time).as_secs_f32();
        self.ui_data.last_imgui_time = now;
        self.ui_data.context.io_mut().delta_time = if delta > 0.0 { delta } else { 1.0 / 60.0 };
    }

    pub fn handle_event(&mut self, window: &Window, event: &winit::event::Event<()>) {
        self.ui_data.platform.handle_event(self.ui_data.context.io_mut(), window, event);

        match event {
            winit::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => unsafe {
                    gl::Viewport(0, 0, size.width as i32, size.height as i32);
                    self.ui_data
                        .scene
                        .lock()
                        .unwrap()
                        .camera
                        .set_aspect(size.width as f32, size.height as f32);
                },
                _ => {}
            },
            _ => {}
        }
    }

    pub fn prepare_frame(&mut self, window: &Window) {
        self.ui_data
            .platform
            .prepare_frame(self.ui_data.context.io_mut(), window)
            .expect("Failed to prepare frame");
    }

    pub fn update_camera(ui: &imgui::Ui, scene: &Arc<Mutex<Scene>>) {
        {
            use imgui::Key;
            use nalgebra as na;

            let speed: f32 = 3.0 * ui.io().delta_time;

            let forward_down = ui.is_key_down(Key::Z);
            let back_down = ui.is_key_down(Key::S);
            let left_down = ui.is_key_down(Key::Q);
            let right_down = ui.is_key_down(Key::D);
            let up_down = ui.is_key_down(Key::Space);
            let down_down = ui.is_key_down(Key::LeftShift);

            if forward_down || back_down || left_down || right_down || up_down || down_down {
                let mut scene = scene.lock().unwrap();
                let cam = &mut scene.camera;

                let rot = na::UnitQuaternion::from_quaternion(cam.transform.rotation);
                let raw_forward: na::Vector3<f32> = rot * na::Vector3::new(0.0, 0.0, -1.0);
                let raw_right: na::Vector3<f32> = rot * na::Vector3::new(1.0, 0.0, 0.0);
                let forward = na::Vector3::new(raw_forward.x, 0.0, raw_forward.z).normalize();
                let right = na::Vector3::new(raw_right.x, 0.0, raw_right.z).normalize();

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
            use nalgebra as na;

            // Only when RMB is held
            let rmb_down = ui.is_mouse_down(imgui::MouseButton::Right);

            static mut LAST_MOUSE: Option<(f32, f32)> = None;

            let sens_deg_per_px: f32 = 0.12;
            let sens = sens_deg_per_px;

            let [mx, my] = ui.io().mouse_pos;

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
                    let mut scene = scene.lock().unwrap();
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
        let now = Instant::now();
        let delta = now.duration_since(self.ui_data.last_fps_time).as_secs_f32();
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

        Self::update_camera(ui, &scene);
        left_panel_ui(ui, &scene, fps_u32, fixed_h);

        let draw_data = self.ui_data.context.render();
        self.ui_data
            .renderer
            .render(self.ui_data.glow_context.as_ref(), &self.ui_data.textures, draw_data)
            .expect("UI render failed");
    }
}

pub trait Inspectable {
    fn get_object_ui(&mut self, ui: &imgui::Ui);
    fn get_object_name(&self) -> String;
}
