use glfw::{Action, Context, Glfw, GlfwReceiver, MouseButton, PWindow, WindowEvent};
use imgui::{Condition, Context as ImGuiContext, Window as ImGuiWindow};
use imgui_opengl_renderer::{Renderer as ImGuiRenderer, Renderer};
use std::ptr::null;
pub struct Ui<'a> {
    pub glfw: Glfw,
    last_imgui_time: f64,
    last_fps_time: f64,
    frames: u64,
    fps: u64,
    context: ImGuiContext,
    renderer: Renderer,
    pub window: &'a mut PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
}

impl<'a> Ui<'a> {
    pub fn init(
        glfw: Glfw,
        window: &'a mut PWindow,
        events: GlfwReceiver<(f64, WindowEvent)>,
    ) -> Self {
        let mut imgui = ImGuiContext::create();
        imgui.set_ini_filename(None);

        let imgui_renderer = ImGuiRenderer::new(&mut imgui, |s| {
            glfw.get_proc_address_raw(s).map_or(null(), |p| p as _)
        });
        let last_imgui_time = glfw.get_time();

        Ui {
            glfw,
            last_imgui_time: last_imgui_time,
            context: imgui,
            renderer: imgui_renderer,
            window,
            events,
            last_fps_time: last_imgui_time,
            frames: 0,
            fps: 0,
        }
    }

    pub fn process_inputs(&mut self) {
        let mut scroll_x: f32 = 0.0;
        let mut scroll_y: f32 = 0.0;
        let mut typed_chars: Vec<char> = Vec::new();

        let now = self.glfw.get_time();
        let delta = (now - self.last_imgui_time) as f32;
        self.last_imgui_time = now;

        let io = self.context.io_mut();
        let (win_w, win_h) = self.window.get_size();
        let (fb_w, fb_h) = self.window.get_framebuffer_size();
        io.delta_time = if delta > 0.0 { delta } else { 1.0 / 60.0 };
        io.display_size = [win_w as f32, win_h as f32];
        io.display_framebuffer_scale = [
            fb_w as f32 / win_w.max(1) as f32,
            fb_h as f32 / win_h.max(1) as f32,
        ];

        let (mx, my) = self.window.get_cursor_pos();
        io.mouse_pos = [mx as f32, my as f32];
        io.mouse_down[0] = self.window.get_mouse_button(MouseButton::Button1) == Action::Press;
        io.mouse_down[1] = self.window.get_mouse_button(MouseButton::Button2) == Action::Press;
        io.mouse_down[2] = self.window.get_mouse_button(MouseButton::Button3) == Action::Press;

        io.mouse_wheel_h = scroll_x;
        io.mouse_wheel = scroll_y;

        for c in typed_chars.drain(..) {
            io.add_input_character(c);
        }

        for (_, event) in glfw::flush_messages(&self.events) {
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
    }

    pub fn render(&mut self) {
        let now = self.glfw.get_time();
        let delta = (now - self.last_fps_time) as f32;
        self.frames += 1;
        if delta >= 1.0 {
            self.fps = self.frames;
            self.frames = 0;
            self.last_fps_time = now;
            println!("{}", self.fps);
        }

        let ui = self.context.frame();
        ImGuiWindow::new(&ui, "Utils")
            .size([400.0, 300.0], Condition::FirstUseEver)
            .build(|| {
                if ui.button("test") {
                    println!("test");
                }
                ui.label_text("FPS", self.fps.to_string());
            });

        self.renderer.render(&mut self.context);

        self.window.swap_buffers();
    }
}
