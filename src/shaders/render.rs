use super::*;

pub struct RenderShader {
    pub program: u32,
}

impl Shader for RenderShader {
    fn new(sources: &[&'static str]) -> Self {
        let mut shader = Self { program: 0 };

        assert!(
            sources.len() == 2,
            "render shader should contain both vertex and fragment shaders"
        );

        let vertex_shader_source = load_file(sources[0]);
        let fragment_shader_source = load_file("shader/fragment_shader.glsl");

        // compile the shader
        unsafe {
            // vertex shader
            let vertex_shader = load_shader(vertex_shader_source, gl::VERTEX_SHADER);

            // fragment shader
            let fragment_shader = load_shader(fragment_shader_source, gl::FRAGMENT_SHADER);

            // link shaders
            let shader_program = create_program(&[vertex_shader, fragment_shader]);

            shader.program = shader_program;
        }

        shader
    }

    unsafe fn get_uniform_location(&self, name: &CStr) -> GLint {
        gl::GetUniformLocation(self.program, name.as_ptr())
    }

    unsafe fn use_program(&self) {
        gl::ClearColor(0., 0., 0., 1.);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // activate the shader
        gl::UseProgram(self.program);

        // update shader uniform
        let color = CString::new("color").unwrap();
        let color_location = self.get_uniform_location(&color);
        gl::Uniform4f(color_location, 0.3, 0.5, 0.1, 1.0);

        // render the geometry
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}

/// Create a screen quad, used as a geometry to render to screen
pub unsafe fn create_screen_quad() -> GLuint {
    // set up vertex data (and buffer(s)) and configure vertex attributes
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
        0.5, -0.5, 0.0, // right
        0.0, 0.5, 0.0, // top
    ];

    let (mut vbo, mut vao) = (0, 0);
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

    // unbind the VAO and vertex buffer
    gl::BindVertexArray(0);
    gl::BindBuffer(gl::ARRAY_BUFFER, 0);

    vao
}
