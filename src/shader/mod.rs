use gl::types::*;
use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    mem,
    os::raw::c_void,
    ptr, str,
};

pub mod compute;
pub mod render;

pub trait Shader {
    fn new(paths: &[&'static str]) -> Self;

    unsafe fn use_program(&self);

    unsafe fn run_program(&self);

    unsafe fn get_uniform_location(&self, name: &str) -> GLint;

    unsafe fn set_bool(&self, name: &str, value: bool) {
        gl::Uniform1i(self.get_uniform_location(name), value as i32);
    }

    unsafe fn set_int(&self, name: &str, value: i32) {
        gl::Uniform1i(self.get_uniform_location(name), value);
    }

    unsafe fn set_float(&self, name: &str, value: f32) {
        gl::Uniform1f(self.get_uniform_location(name), value);
    }

    unsafe fn set_vector3(&self, name: &str, value: &Vec<f32>) {
        gl::Uniform3fv(self.get_uniform_location(name), 1, value.as_ptr());
    }

    unsafe fn set_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
        gl::Uniform3f(self.get_uniform_location(name), x, y, z);
    }

    unsafe fn set_vec4(&self, name: &str, x: f32, y: f32, z: f32, w: f32) {
        gl::Uniform4f(self.get_uniform_location(name), x, y, z, w);
    }

    unsafe fn set_vector4(&self, name: &str, value: &Vec<f32>) {
        gl::Uniform4fv(self.get_uniform_location(name), 1, value.as_ptr());
    }

    unsafe fn bind_image(
        &self,
        image: GLuint,
        unit: GLuint,
        access_flag: GLenum,
        format_flag: GLenum,
    ) {
        gl::BindImageTexture(unit, image, 0, gl::FALSE, 0, access_flag, format_flag);
    }

    unsafe fn bind_texture(&self, texture: GLuint, unit: GLuint) {
        gl::ActiveTexture(gl::TEXTURE0 + unit);
        gl::BindTexture(gl::TEXTURE_2D, texture);
    }

    unsafe fn bind_texture_uniform(&self, name: &str, unit: GLint) {
        // use the program to be able to set the uniform value
        self.use_program();

        // set the uniform
        self.set_int(name, unit);

        // unload the program
        gl::UseProgram(0);
    }
}

fn check_for_compile_error(shader: GLuint, kind: GLenum) -> Result<(), String> {
    let mut success = gl::FALSE as GLint;

    unsafe {
        match kind {
            gl::SHADER => {
                gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            }
            gl::PROGRAM => {
                gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            }
            _ => {
                panic!("should be either a shader or a program");
            }
        }
    }

    if success != gl::TRUE as GLint {
        let mut len: GLint = 0;

        unsafe { gl::GetProgramiv(shader, gl::INFO_LOG_LENGTH, &mut len) };

        // allocate the buffer
        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);

        // fill it with len spaces
        buffer.extend([b' '].iter().cycle().take(len as usize));

        unsafe {
            // convert buffer to CString
            let error = CString::from_vec_unchecked(buffer);

            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );

            return Err(error.to_string_lossy().into_owned());
        }
    }

    Ok(())
}

fn load_file(path: &str) -> CString {
    let mut file = File::open(path).unwrap_or_else(|_| panic!("Failed to open {}", path));
    let mut code = String::new();

    file.read_to_string(&mut code)
        .expect("Failed to read vertex shader");

    CString::new(code.as_bytes()).unwrap()
}

unsafe fn load_shader(source: CString, shader_kind: GLenum) -> GLuint {
    let shader = gl::CreateShader(shader_kind);

    gl::ShaderSource(shader, 1, &source.as_ptr(), ptr::null());
    gl::CompileShader(shader);

    match check_for_compile_error(shader, gl::SHADER) {
        Err(error) => panic!("\nError loading the shader:\n{}\n", error),
        Ok(()) => println!("shader loaded."),
    }

    shader
}

unsafe fn create_program(shaders: &[GLuint]) -> GLuint {
    let shader_program = gl::CreateProgram();

    for shader in shaders {
        gl::AttachShader(shader_program, *shader);
    }

    gl::LinkProgram(shader_program);

    match check_for_compile_error(shader_program, gl::PROGRAM) {
        Err(error) => panic!("\nError creating the program:\n{}\n", error),
        Ok(()) => println!("program linked."),
    }

    for shader in shaders {
        gl::DeleteShader(*shader);
    }

    shader_program
}

fn c_str_from(name: &str) -> &CStr {
    unsafe { CStr::from_bytes_with_nul_unchecked(name.as_bytes()) }
}
