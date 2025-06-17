extern crate gl;
extern crate glfw;

use crate::polygon;
use crate::run::mesh::Polygon;
use crate::run::shader_program::{ShaderProgram, Uniform};
use glfw::{Action, Context, Key};
use mesh::Mesh;
use std::f32::consts::TAU;
use vatnar_linalg::{Point2, Vector2};

mod mesh;
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
    let gamma_uniform = shader_program.get_uniform("u_gamma");
    let mut gamma: f32 = 0.5;

    let meshes = define_meshes();

    unsafe {
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    }

    unsafe {
        gl::Uniform3f(color_uniform.into(), 1.0, 0.2, 0.5);
    }
    // main loop
    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&window_event_receiver) {
            println!("Event gotten: {event:?}");
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(Key::Up, _, Action::Press | Action::Repeat, _) => {
                    gamma = (gamma + 0.1).clamp(0.0, 1.0)
                }
                glfw::WindowEvent::Key(Key::Down, _, Action::Press | Action::Repeat, _) => {
                    gamma = (gamma - 0.1).clamp(0.0, 1.0)
                }
                _ => {}
            }
        }

        unsafe {
            gl::Uniform1f(time_uniform.into(), glfw.get_time() as f32); // update u_time
            gl::Uniform1f(gamma_uniform.into(), gamma);
        }

        render(&meshes, color_uniform);
        window.swap_buffers();
    }

    Ok(())
}

fn define_meshes() -> Vec<Mesh> {
    let trapezoid = polygon![
        Point2::new(-0.8, -0.16),
        Point2::new(0.64, 0.3),
        Point2::new(0.4, 0.4),
        Point2::new(-0.4, 0.4),
    ];

    let tri = polygon![
        Point2::new(-0.64, -0.8),
        Point2::new(0.0, -0.2),
        Point2::new(0.64, -0.8),
    ];

    let n = 5; // Try 10 points
    let radius = 0.5;

    let points: Vec<Point2<f32>> = (0..n)
        .map(|i| {
            let angle = i as f32 * TAU / n as f32;
            Point2::new((angle.cos() * radius), angle.sin() * radius)
        })
        .collect();

    for p in &points {
        println!("{:?}", p);
    }

    let testpoints = Polygon(points);

    // Mesh creation needs abstraction
    vec![
        // Mesh::from_polygon(trapezoid, gl::POINTS, (1.0, 0.0, 0.0).into()),
        // Mesh::from_polygon(tri, gl::POINTS, (0.0, 1.0, 0.0).into()),
        Mesh::from_polygon(testpoints, gl::POINTS, (1.0, 0.0, 0.0).into()),
    ]
}

fn render(meshes: &Vec<Mesh>, color: Uniform) {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);

        // Draw meshes
        for mesh in meshes {
            unsafe {
                let (r, g, b) = mesh.color.into();
                gl::Uniform3f(color.into(), r, g, b);
            }
            mesh.draw();
        }
    }
}

#[cfg(test)]
mod tests {}
