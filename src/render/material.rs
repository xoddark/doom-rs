use super::doom_gl::gl;
use std::io::Read;
use std::{fs::File, path::Path};

pub struct Material {
    vs: u32,
    fs: u32,
    program: u32,
}

const WALL_FRAG_STR: &str = include_str!("wall.frag");
const WALL_VERT_STR: &str = include_str!("wall.vert");

pub fn create_shader(gl: &gl::Gl, content: &str, shader_type: gl::types::GLenum) -> u32 {
    let length = content.len() as i32;

    let vs;
    unsafe {
        vs = gl.CreateShader(shader_type);
        gl.ShaderSource(vs, 1, [content.as_ptr() as *const _].as_ptr(), &length);
        gl.CompileShader(vs);

        let mut status = 0;
        gl.GetShaderiv(vs, gl::COMPILE_STATUS, &mut status);
        if status != 1 {
            let mut message = [0u8; 1024];
            let mut length = 0;
            gl.GetShaderInfoLog(vs, 1024, &mut length, message.as_mut_ptr() as *mut _);

            let str = String::from_raw_parts(message.as_mut_ptr(), length as usize, 1024);
            panic!("{}", str);
        }
    }
    vs
}

impl Material {
    pub fn new(gl: &gl::Gl) -> Self { // , vs: &str, fs: &str
        let vs = create_shader(gl, WALL_VERT_STR, gl::VERTEX_SHADER); //Path::new(vs)
        let fs = create_shader(gl, WALL_FRAG_STR, gl::FRAGMENT_SHADER); // Path::new(fs)

        let program = unsafe { gl.CreateProgram() };
        unsafe {
            gl.AttachShader(program, vs);
            gl.AttachShader(program, fs);
            gl.LinkProgram(program);
        }

        Material { vs, fs, program }
    }

    pub fn get_uniform_location(&self, gl: &gl::Gl, name: &str) -> i32 {
        let location = unsafe { gl.GetUniformLocation(self.program, name.as_ptr() as *const _) };
        unsafe { assert!(gl.GetError() == 0) };
        location
    }

    pub fn get_attrib_location(&self, gl: &gl::Gl, name: &str) -> i32 {
        let location = unsafe { gl.GetAttribLocation(self.program, name.as_ptr() as *const _) };
        unsafe { assert!(gl.GetError() == 0) };
        location
    }

    pub fn bind(&self, gl: &gl::Gl) {
        unsafe { gl.UseProgram(self.program) };
    }
}

/*
impl Drop for Material {
    fn drop(&mut self) {
        unsafe {
            self.gl.DetachShader(self.program, self.vs);
            self.gl.DetachShader(self.program, self.fs);
            self.gl.DeleteProgram(self.program);
            self.gl.DeleteShader(self.vs);
            self.gl.DeleteShader(self.fs);
        }
    }
}
*/
