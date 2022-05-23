extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::Context;

mod shaders;
mod utils;

use shaders::{render::*, Shader};
use utils::*;

fn main() {
    // Initialize and configureGLFW
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(1024, 1024, "Game of Life", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load OpenGL
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Load render shader and geometry
    let render_shader =
        RenderShader::new(&["shader/vertex_shader.glsl", "shader/vertex_shader.glsl"]);
    let screen_quad: GLuint;
    unsafe {
        // load the shader used to render to screen

        // create the screen geometry and bind it
        screen_quad = create_screen_quad();
        gl::BindVertexArray(screen_quad);
    }

    // Loop until the user closes the window
    while !window.should_close() {
        // events
        process_events(&mut window, &events);

        // render pass
        unsafe {
            render_shader.use_program();
        }

        // swap buffers and poll events
        window.swap_buffers();
        glfw.poll_events();
    }
}
