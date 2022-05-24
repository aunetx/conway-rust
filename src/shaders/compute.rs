use super::*;

pub struct ComputeShader {
    pub program: u32,
}

impl Shader for ComputeShader {
    fn new(paths: &[&'static str]) -> Self {
        let mut shader = Self { program: 0 };

        let mut shader_sources: Vec<CString> = Vec::new();

        for path in paths {
            shader_sources.push(load_file(path))
        }

        // compile the shader
        unsafe {
            let mut shaders: Vec<GLuint> = Vec::new();

            // load each shader
            let mut i = 1;
            for shader_source in shader_sources {
                print!("Loading compute shader number {}... ", i);
                shaders.push(load_shader(shader_source, gl::COMPUTE_SHADER));
                i += 1;
            }

            // link shaders
            print!("Linking compute shader... ");
            let shader_program = create_program(&shaders);

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
