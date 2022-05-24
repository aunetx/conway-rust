use glfw::{Action, Key, MouseButtonLeft, Window, WindowEvent};
use std::sync::mpsc::Receiver;
use std::time::Instant;

#[derive(Debug)]
pub struct State {
    instant: Instant,
    pub frame_index: u32,
    pub time: f32,
    pub window: (i32, i32),
    /// The position of the mouse
    pub mouse_position: (f64, f64),
    /// The state of the mouse: true for pressed
    pub mouse_state: bool,
}

impl State {
    pub fn new(window: &Window) -> Self {
        Self {
            instant: Instant::now(),
            frame_index: 0,
            time: 0.,
            window: window.get_size(),
            mouse_position: {
                let window_size = window.get_size();
                let mouse_position = window.get_cursor_pos();

                (
                    mouse_position.0 / window_size.0 as f64,
                    1. - mouse_position.1 / window_size.1 as f64,
                )
            },
            mouse_state: window.get_mouse_button(MouseButtonLeft) == Action::Press,
        }
    }

    fn update_mouse_position(&mut self, x: f64, y: f64, window: &Window) {
        let window_size = window.get_size();

        self.mouse_position = (x / window_size.0 as f64, 1. - y / window_size.1 as f64);
    }

    pub fn process_events(&mut self, events: &Receiver<(f64, WindowEvent)>, window: &mut Window) {
        for (_, event) in glfw::flush_messages(events) {
            match event {
                WindowEvent::FramebufferSize(width, height) => {
                    // set opengl viewport to windows size
                    unsafe { gl::Viewport(0, 0, width, height) };

                    // update the state for the window datas
                    self.window = (width, height);
                }
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    // close the window if `esc` is pressed
                    window.set_should_close(true)
                }
                WindowEvent::CursorPos(x, y) => {
                    // set the mouse position
                    self.update_mouse_position(x, y, window);
                }
                WindowEvent::MouseButton(button, _, _) => {
                    if button == MouseButtonLeft {
                        self.mouse_state = window.get_mouse_button(MouseButtonLeft) == Action::Press
                    }
                }
                _ => {}
            }
        }
    }

    pub fn update_time(&mut self) {
        self.time = self.instant.elapsed().as_secs_f32();
    }
}
