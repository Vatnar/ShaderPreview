extern crate gl;
extern crate glfw;

pub use crate::polygon;
use gl::TRIANGLES;
use gl::types::GLfloat;
use glfw::{Action, Context, Key};
use mesh::Mesh;
use shader_program::{ShaderProgram, Uniform};
use vatnar_linalg::Vector2;

pub(crate) mod mesh;
mod shader_program;
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut glfw = glfw::init(glfw::fail_on_errors)?;
    let window_size = Vector2::new(800, 600);

    // create window and events receiver
    let (mut window, window_event_receiver) = glfw
        .create_window(
            window_size.x,
            window_size.y,
            "ShaderPreview",
            glfw::WindowMode::Windowed,
        )
        .ok_or("Failed to create GLFW window.")?;

    // set window to current on thread, and turn on key polling
    window.make_current();
    window.set_key_polling(true);
    window.set_scroll_polling(true);
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1)); // v-sync

    // Initializes OpenGL function pointers by querying their addresses from the current context.
    // This must be called after creating the OpenGL context, or OpenGL functions won't work.
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
    unsafe {
        gl::Enable(gl::PROGRAM_POINT_SIZE);
    }

    // Compile shaders
    let vertex_src = include_str!("vertex_shader.glsl");
    let fragment_src = include_str!("fragment_shader.glsl");
    let shader_program = ShaderProgram::from_source(vertex_src, fragment_src)?;
    shader_program.use_program();

    // Blending
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    // Setup uniforms
    let time_uniform = shader_program.get_uniform("u_time");
    let color_uniform = shader_program.get_uniform("u_color");

    let offset_uniform = shader_program.get_uniform("u_offset");
    let zoom_uniform = shader_program.get_uniform("u_zoom");

    // The offset sort of acts like moving a camera
    // TODO MARK figure out "zooming"
    let mut offset: Vector2<f64> = Vector2::new(0.0, 0.0);
    let mut zoom = 1.0;

    let meshes = define_meshes();

    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    }

    unsafe {
        gl::Uniform3f(*color_uniform, 1.0, 0.2, 0.5);
        gl::Uniform2f(*offset_uniform, 1.0, 0.0);
    }

    let mut offset_keys = Vector2::new(0, 0);

    // main loop
    while !window.should_close() {
        glfw.poll_events();

        // TODO change so it checks for releases and stuff
        // instead so you can hold both left and down fir instance and it work
        for (_, event) in glfw::flush_messages(&window_event_receiver) {
            // println!("Event gotten: {event:?}"); // DEBUG

            use glfw::WindowEvent::{Key as glfwKey, Scroll};
            match event {
                glfwKey(Key::Escape, _, Action::Press, _) => window.set_should_close(true),

                // Camera movement
                glfwKey(Key::Left | Key::A, _, action, _) => match action {
                    Action::Press => offset_keys.x += 1,
                    Action::Release => offset_keys.x -= 1,
                    _ => {}
                },

                glfwKey(Key::Right | Key::D, _, action, _) => match action {
                    Action::Press => offset_keys.x -= 1,
                    Action::Release => offset_keys.x += 1,
                    _ => {}
                },

                glfwKey(Key::Up | Key::W, _, action, _) => match action {
                    Action::Press => offset_keys.y -= 1,
                    Action::Release => offset_keys.y += 1,
                    _ => {}
                },

                glfwKey(Key::Down | Key::S, _, action, _) => match action {
                    Action::Press => offset_keys.y += 1,
                    Action::Release => offset_keys.y -= 1,
                    _ => {}
                },

                // Zoom
                Scroll(_, y_offset) => {
                    zoom += y_offset;
                    if zoom as i32 == 0 {
                        zoom += y_offset
                    }
                }
                _ => {}
            }
        }

        unsafe {
            gl::Uniform1f(*time_uniform, glfw.get_time() as f32); // update u_time
            gl::Uniform2f(*offset_uniform, offset.x as GLfloat, offset.y as GLfloat);
            let scale = if zoom < 0.0 { -1.0 / zoom } else { zoom };
            offset += offset_keys.normalized_i32() * 0.01 * (1.0 / scale);

            gl::Uniform1f(*zoom_uniform, scale as GLfloat)
        }

        render(&meshes, color_uniform);
        window.swap_buffers();
    }

    Ok(())
}

fn define_meshes() -> Vec<Mesh> {
    let triangle1 = polygon![-0.5, -0.5, 0.0, 0.5, 0.5, -0.5];
    let triangle2 = polygon![-0.8, 0.2, -0.3, 0.9, 0.2, 0.3];
    let triangle3 = polygon![5.0, 5.0, 6.0, 5.0, 5.5, 6.0];
    let triangle4 = polygon![-7.0, 4.0, -6.0, 4.0, -6.5, 5.0];
    let triangle5 = polygon![3.0, -5.0, 4.0, -5.0, 3.5, -6.0];
    let triangle6 = polygon![-4.0, -4.0, -3.0, -4.0, -3.5, -5.0];

    vec![
        Mesh::from_polygon(triangle1, TRIANGLES, (1.0, 0.0, 0.0).into()),
        Mesh::from_polygon(triangle2, TRIANGLES, (0.0, 1.0, 0.0).into()),
        Mesh::from_polygon(triangle3, TRIANGLES, (0.0, 0.0, 1.0).into()),
        Mesh::from_polygon(triangle4, TRIANGLES, (1.0, 1.0, 0.0).into()),
        Mesh::from_polygon(triangle5, TRIANGLES, (0.0, 1.0, 1.0).into()),
        Mesh::from_polygon(triangle6, TRIANGLES, (1.0, 1.0, 1.0).into()),
    ]
}

fn render(meshes: &Vec<Mesh>, color: Uniform) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    // Draw meshes
    for mesh in meshes {
        unsafe {
            let (r, g, b) = mesh.color.into();
            gl::Uniform3f(color.into(), r, g, b);
        }
        mesh.draw();
    }
}
