use gl::types::*;
use std::{
    ffi::{CStr, CString},
    fs::File,
    io::Read,
    mem,
    os::raw::c_void,
    ptr, str,
};

pub mod render;

pub trait Shader {
    fn new(sources: &[&'static str]) -> Self;

    unsafe fn use_program(&self);

    unsafe fn get_uniform_location(&self, name: &CStr) -> GLint;

    unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(self.get_uniform_location(name), value as i32);
    }

    unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(self.get_uniform_location(name), value);
    }

    unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(self.get_uniform_location(name), value);
    }

    unsafe fn set_vector3(&self, name: &CStr, value: &Vec<f32>) {
        gl::Uniform3fv(self.get_uniform_location(name), 1, value.as_ptr());
    }

    unsafe fn set_vec3(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(self.get_uniform_location(name), x, y, z);
    }

    unsafe fn set_vec4(&self, name: &CStr, x: f32, y: f32, z: f32, w: f32) {
        gl::Uniform4f(self.get_uniform_location(name), x, y, z, w);
    }

    unsafe fn set_vector4(&self, name: &CStr, value: &Vec<f32>) {
        gl::Uniform4fv(self.get_uniform_location(name), 1, value.as_ptr());
    }
}

unsafe fn check_for_compile_error(shader: GLuint, kind: GLenum) {
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(1024);
    // FIXME crashes if set, does not display the error if not
    //info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character

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

    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            shader,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "Shader compilation failed:\n{:?}",
            str::from_utf8(&info_log).unwrap()
        );
    }
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

    check_for_compile_error(shader, gl::SHADER);

    shader
}

unsafe fn create_program(shaders: &[GLuint]) -> GLuint {
    let shader_program = gl::CreateProgram();

    for shader in shaders {
        gl::AttachShader(shader_program, *shader);
    }

    gl::LinkProgram(shader_program);

    check_for_compile_error(shader_program, gl::PROGRAM);

    for shader in shaders {
        gl::DeleteShader(*shader);
    }

    shader_program
}
