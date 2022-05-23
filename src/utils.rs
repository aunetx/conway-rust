use glfw::{Action, Key};
use std::sync::mpsc::Receiver;

pub fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // set opengl viewport to windows size
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                // close the window if `esc` is pressed
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
