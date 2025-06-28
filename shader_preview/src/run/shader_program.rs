use std::ffi::{CStr, CString};
use std::ops::Deref;

pub struct ShaderProgram {
    pub(crate) id: u32,
}
#[derive(Clone, Copy)]
pub struct Uniform(gl::types::GLint);

impl From<Uniform> for i32 {
    fn from(value: Uniform) -> Self {
        value.0
    }
}

impl Deref for Uniform {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ShaderProgram {
    pub fn from_source(vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
        let vertex_shader_src =
            CString::new(vertex_src).map_err(|_| "Invalid vertex shader source")?;
        let fragment_shader_src =
            CString::new(fragment_src).map_err(|_| "Invalid fragment shader source")?;

        let vertex_shader = ShaderProgram::compile_shader(&vertex_shader_src, gl::VERTEX_SHADER);
        let fragment_shader =
            ShaderProgram::compile_shader(&fragment_shader_src, gl::FRAGMENT_SHADER);

        let shader_program = ShaderProgram::link_program(vertex_shader, fragment_shader);

        Ok(ShaderProgram { id: shader_program })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn get_uniform(&self, uniform_name: &str) -> Uniform {
        unsafe {
            Uniform(gl::GetUniformLocation(
                self.id,
                CString::new(uniform_name)
                    .unwrap_or_else(|_| {
                        panic!(
                            "Couldn't create CString for uniform variable '{}'",
                            uniform_name
                        )
                    })
                    .as_ptr(),
            ))
        }
    }

    fn link_program(vs: u32, fs: u32) -> u32 {
        unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vs);
            gl::AttachShader(program, fs);
            gl::LinkProgram(program);

            // errors
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::new(vec![b' '; len as usize]).unwrap();
                gl::GetProgramInfoLog(program, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
                panic!("Program linking error: {:?}", error);
            }

            gl::DeleteShader(vs);
            gl::DeleteShader(fs);

            program
        }
    }
    fn compile_shader(src: &CStr, ty: u32) -> u32 {
        unsafe {
            let shader = gl::CreateShader(ty);
            gl::ShaderSource(shader, 1, &src.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            // error check
            let mut success = gl::FALSE as gl::types::GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let error = CString::new(vec![b' '; len as usize]).unwrap();
                gl::GetShaderInfoLog(shader, len, std::ptr::null_mut(), error.as_ptr() as *mut _);
                // TODO MARK this needs to be thrown up somehow later
                panic!("Shader compilation error: {:?}", error);
            }

            shader
        }
    }
}
