mod load_model;
mod shd_prg;
mod tex;
mod var_loc;
mod vex;

pub use shd_prg::init_shader_programs;
pub use tex::init_textures;
pub use var_loc::get_variable_locations;
pub use vex::init_vertex_array_objects;

use glfw::{
    fail_on_errors, Callback, Context, CursorMode, Error, Glfw, OpenGlProfileHint, Window,
    WindowEvent, WindowHint,
};
use opengl_learn::gl;
use std::sync::mpsc::Receiver;

pub fn init_glfw() -> (Glfw, Window, Receiver<(f64, WindowEvent)>) {
    println!("GLFW run-time version: {}", glfw::get_version());
    let error_callback = Some(Callback {
        f: fail_on_errors as fn(Error, String, &()),
        data: (),
    });
    let mut glfw = glfw::init(error_callback).unwrap();
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    let (mut window, events) = glfw
        .create_window(600, 500, "OpenGL learn", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");
    window.make_current();
    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_cursor_enter_polling(true);
    //window.set_focus_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(CursorMode::Disabled);
    window.set_cursor_pos(
        window.get_size().0 as f64 / 2.0,
        window.get_size().1 as f64 / 2.0,
    );
    (glfw, window, events)
}

pub fn init_open_gl(window: &mut Window) {
    gl::load_with(|s| window.get_proc_address(s));
    unsafe {
        gl::Viewport(0, 0, window.get_size().0, window.get_size().1);
        println!("Viewport is loaded: {}", gl::Viewport::is_loaded());
        let mut max_vertex_attributes = 0;
        gl::GetIntegerv(gl::MAX_VERTEX_ATTRIBS, &mut max_vertex_attributes);
        println!(
            "Max of vertex attributes supported: {}",
            max_vertex_attributes
        );
    }
}
