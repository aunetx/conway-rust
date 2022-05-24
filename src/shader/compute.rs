use super::*;

#[derive(Default)]
pub struct ComputeShader {
    pub program: u32,
    pub compute_group: (u32, u32, u32),
}

impl Shader for ComputeShader {
    fn new(paths: &[&'static str]) -> Self {
        let mut shader = Self::default();

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

    unsafe fn use_program(&self) {
        // activate the shader
        gl::UseProgram(self.program);
    }

    unsafe fn run_program(&self) {
        let c = self.compute_group;

        // dispach the computation
        gl::DispatchCompute(c.0, c.1, c.2);

        // pause CPU execution until the images are closed
        gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
    }

    unsafe fn get_uniform_location(&self, name: &str) -> GLint {
        gl::GetUniformLocation(self.program, c_str_from(name).as_ptr())
    }
}
