extern crate gl;
extern crate glfw;

use glfw::Context;

mod events;
mod shader;
mod texture;

use events::*;
use shader::{compute::*, render::*, Shader};
use texture::*;

const DEBUG: bool = true;

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
    window.set_cursor_pos_polling(true);
    window.set_framebuffer_size_polling(true);
    window.set_mouse_button_polling(true);
    window.set_close_polling(true);

    // Load OpenGL
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Create the render shader
    let render_shader = RenderShader::new(&["shader/render.vert", "shader/render.frag"]);

    // Load the screen geometry
    let _ = unsafe { load_screen_quad() };

    // Create the compute shaders
    let mut compute_shader = ComputeShader::new(&["shader/conway.comp"]);
    let mut copy_shader = ComputeShader::new(&["shader/copy.comp"]);

    // Create an load textures
    let current_generation = new_texture(ImageSource::Path("texture/first_generation.png"), gl::R8);
    let next_generation = new_texture(ImageSource::Empty(1024, 1024), gl::R8);

    // Bind texture to currect spot, needed only once
    unsafe {
        render_shader.bind_texture_uniform("current_generation", 0);
    }

    // Create the struct that will hold the state of the program
    let mut state = State::new(&window);

    println!("Beginning main loop!\n");

    // Loop until the user closes the window
    while !window.should_close() {
        // update the state
        state.process_events(&events, &mut window);
        state.update_time();

        if DEBUG && state.frame_index % 60 == 0 {
            println!(
                "\nFrame n°{}, time = {} seconds",
                state.frame_index, state.time
            );
            dbg!(&state);
        }

        // compute pass
        unsafe {
            compute_shader.use_program();

            compute_shader.bind_image(current_generation, 0, gl::READ_WRITE, gl::R8);
            compute_shader.bind_image(next_generation, 1, gl::READ_WRITE, gl::R8);

            compute_shader.set_float("mouse_radius", 5.);
            compute_shader.set_float("time", state.time);
            compute_shader.set_vector2(
                "mouse_position",
                &[state.mouse_position.0 as f32, state.mouse_position.1 as f32],
            );
            compute_shader.set_bool("mouse_pressed", state.mouse_state);

            compute_shader.compute_group = (1024 / 8, 1024 / 8, 1);
            compute_shader.run_program();
        }

        // render pass
        unsafe {
            render_shader.use_program();

            render_shader.bind_texture(current_generation, 0);

            render_shader.run_program();
        }

        // copy pass
        unsafe {
            copy_shader.use_program();

            copy_shader.bind_image(next_generation, 0, gl::READ_WRITE, gl::R8);
            copy_shader.bind_image(current_generation, 1, gl::READ_WRITE, gl::R8);

            copy_shader.compute_group = (1024 / 8, 1024 / 8, 1);
            copy_shader.run_program();
        }

        // swap buffers and poll events
        window.swap_buffers();
        glfw.poll_events();

        state.frame_index += 1;
    }
}
