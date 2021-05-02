mod shd_prg;
mod tex;
mod var_loc;
mod vex;

pub use shd_prg::init_shader_programs;
pub use tex::init_textures;
#[allow(deprecated)]
pub use var_loc::get_variable_locations;
pub use var_loc::get_variable_locations_2;
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
    //glfw.window_hint(WindowHint::Samples(Some(4)));
    let (mut window, events) = glfw
        .create_window(700, 600, "OpenGL learn", glfw::WindowMode::Windowed)
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
        let parameters = [
            (gl::MAX_VERTEX_ATTRIBS, "Max vertex attributes supported"),
            (
                gl::MAX_VERTEX_UNIFORM_COMPONENTS,
                "Max vertex uniform components supported",
            ),
            (
                gl::MAX_FRAGMENT_UNIFORM_COMPONENTS,
                "Max fragment uniform components supported",
            ),
        ];
        for (param, msg) in &parameters {
            let mut value = 0;
            gl::GetIntegerv(*param, &mut value);
            println!("{}: {}", msg, value);
        }
    }
}
