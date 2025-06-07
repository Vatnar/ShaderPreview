extern crate gl;
extern crate glfw;

use glfw::{Action, Context, Key};
use std::ffi::{CStr, CString};

struct Mesh {
    vao: u32,
    vbo: u32,
    vertex_count: usize,
    draw_mode: u32,
}

// TODO USE LINALG LIBRARY INSTEAD OF VERTEX ARRAY
impl Mesh {
    fn new(vertices: &[f32], draw_mode: gl::types::GLenum) -> Self {
        // 1. Generate and bind a Vertex Array Object VAO
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao); // Store ID to generated VAO in variable
            gl::BindVertexArray(vao); // Bind the VAO so opengl knows we setting up
        }

        // 2. Generate and bind a vertex buffer object VBO
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo); // Generate one vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo); // Bind it as an array buffer
            gl::BufferData(
                // Upload the data to GPU
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as isize, // Size of data in bytes
                vertices.as_ptr() as *const _,                // Pointer to data
                gl::DYNAMIC_DRAW,                             // Tell gpu we dont update often
            );
        }

        // 3. Describe how the data in vbo is laid out
        unsafe {
            gl::VertexAttribPointer(
                0,                                     // Attribute index (loc 0 in shader)
                2,                                     // 2 Components per vertex (x, y)
                gl::FLOAT,                             // Data type is float
                gl::FALSE,                             // Dont normalize
                2 * std::mem::size_of::<f32>() as i32, // Stride, 2 floats per vertex
                std::ptr::null(),                      // offset: start at the beginning
            );
            gl::EnableVertexAttribArray(0); // enable specified attributre
        }
        Mesh {
            vao,
            vbo,
            vertex_count: vertices.len() / 2,
            draw_mode,
        }
    }
    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(self.draw_mode, 0, self.vertex_count as i32);
        }
    }
}
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::fail_on_errors)?;

    let (mut window, events) = glfw
        .create_window(800, 600, "ShaderPreview", glfw::WindowMode::Windowed)
        .ok_or("Failed to create GLFW window.")?;

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    let meshes = define_meshes();
    let vertex_src = include_str!("vertex_shader.glsl");
    let fragment_src = include_str!("fragment_shader.glsl");
    let shader_program = ShaderProgram::from_source(vertex_src, fragment_src)?;

    shader_program.use_program();
    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    }
    let time_location = unsafe {
        gl::GetUniformLocation(shader_program.id, CString::new("u_time").unwrap().as_ptr())
    };

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        unsafe {
            gl::UseProgram(shader_program.id);
            gl::Uniform1f(time_location, glfw.get_time() as f32);
        }

        render(&meshes);
        window.swap_buffers();
    }

    Ok(())
}

fn define_meshes() -> Vec<Mesh> {
    let trapezoid: [f32; 8] = [
        -0.8, -0.16, // bottom left
        0.64, -0.0, // bottom right
        0.4, 0.4, // top right
        -0.4, 0.4, // top left
    ];

    let tri: [f32; 6] = [
        -0.64, -0.8, // bottom left
        0.0, -0.2, // top
        0.64, -0.8, // bottom right
    ];

    vec![
        Mesh::new(&trapezoid, gl::TRIANGLE_FAN),
        Mesh::new(&tri, gl::TRIANGLE_FAN),
        Mesh::new(&tri, gl::QUADS),
    ]
}

pub struct ShaderProgram {
    id: u32,
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
                panic!("Shader compilation error: {:?}", error);
            }

            shader
        }
    }
}

fn render(meshes: &Vec<Mesh>) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Draw meshes
        for mesh in meshes {
            mesh.draw();
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

#[cfg(test)]
mod tests {}
