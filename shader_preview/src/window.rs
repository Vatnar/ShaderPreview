extern crate gl;
extern crate glfw;

use gl::types::*;
use glfw::{Action, Context, Key, PWindow};

pub fn runer() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let (mut window, events) = glfw
        .create_window(800, 600, "Cool Window thing", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        render(&mut window);
    }
}

fn render(window: &mut PWindow) {
    unsafe {
        gl::ClearColor(
            0.703125 as GLfloat,
            0.7421875 as GLfloat,
            0.95703125 as GLfloat,
            1.0,
        );

        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
    window.swap_buffers();
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}

#[cfg(test)]
mod tests {}
