use super::*;

pub struct RenderShader {
    pub program: u32,
}

impl Shader for RenderShader {
    fn new(paths: &[&'static str]) -> Self {
        let mut shader = Self { program: 0 };

        assert!(
            paths.len() == 2,
            "render shader should contain both vertex and fragment shaders"
        );

        let vertex_shader_source = load_file(paths[0]);
        let fragment_shader_source = load_file(paths[1]);

        // compile the shader
        unsafe {
            // vertex shader
            print!("Loading vertex shader... ");
            let vertex_shader = load_shader(vertex_shader_source, gl::VERTEX_SHADER);

            // fragment shader
            print!("Loading fragment shader... ");
            let fragment_shader = load_shader(fragment_shader_source, gl::FRAGMENT_SHADER);

            // link shaders
            print!("Linking render shader... ");
            let shader_program = create_program(&[vertex_shader, fragment_shader]);

            shader.program = shader_program;
        }

        shader
    }

    unsafe fn get_uniform_location(&self, name: &str) -> GLint {
        gl::GetUniformLocation(self.program, c_str_from(name).as_ptr())
    }

    unsafe fn use_program(&self) {
        gl::ClearColor(0., 0., 0., 1.);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // activate the shader
        gl::UseProgram(self.program);

        // update shader uniform
        self.set_vec4("color", 0.3, 0.5, 0.1, 1.0);

        // render the geometry
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

/// Create and load directly a screen quad, used as a geometry to render to screen
pub fn load_screen_quad() {
    // set up vertex data (and buffer(s)) and configure vertex attributes
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let (mut vbo, mut vao) = (0, 0);

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        // bind the Vertex Array Object
        gl::BindVertexArray(vao);

        // bind and set vertex buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        // configure vertex attributes
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        // unbind the vertex buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }
}
