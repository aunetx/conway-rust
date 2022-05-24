extern crate gl;
extern crate glfw;

use glfw::Context;

mod shader;
mod texture;
mod utils;

use shader::{compute::*, render::*, Shader};
use texture::*;
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

    // Create the render shader
    let render_shader =
        RenderShader::new(&["shader/vertex_shader.glsl", "shader/fragment_shader.glsl"]);

    // Load the screen geometry
    load_screen_quad();

    // Create the compute shader
    let mut compute_shader = ComputeShader::new(&["shader/conway_shader.glsl"]);

    // Create an load textures
    let current_generation = new_texture(ImageSource::Path("texture/first_generation.png"), gl::R8);
    //let next_generation = new_texture(ImageSource::Empty(1024, 1024), gl::R8);

    // bind texture to currect spot, needed only once
    unsafe {
        render_shader.bind_texture_uniform("current_generation", 0);
    }

    println!("Beginning main loop!");

    // Loop until the user closes the window
    while !window.should_close() {
        // events
        process_events(&mut window, &events);

        // compute pass
        unsafe {
            compute_shader.use_program();
            compute_shader.compute_group = (1024 / 8, 1024 / 8, 1);
            compute_shader.bind_image(current_generation, 0, gl::READ_WRITE, gl::R8);
            //compute_shader.bind_image(next_generation, 1, gl::READ_WRITE, gl::R8);
            compute_shader.run_program();
        }

        // render pass
        unsafe {
            render_shader.use_program();
            render_shader.bind_texture(current_generation, 0);
            render_shader.run_program();
        }

        // swap buffers and poll events
        window.swap_buffers();
        glfw.poll_events();
    }
}
