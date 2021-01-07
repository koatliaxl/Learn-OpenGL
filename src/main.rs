mod camera;
mod draw;
mod init;
mod load_tex;
mod process_input;
mod shaders;
mod state_and_cfg;

pub use ::opengl_learn::gl;

use self::draw::*;
use self::init::*;
use self::process_input::process_input;
use self::state_and_cfg::*;
use glfw::Context;
use opengl_learn::Model;
use std::time::Instant;

const _NULL: *const i32 = std::ptr::null();

fn main() {
    let (mut glfw, mut window, events) = init_glfw();
    init_open_gl(&mut window);
    let mut state = State::new(window.get_size());
    let cfg = Config::new();
    let gfx = GlData::new();
    let mut model = Model::new();
    init_draw(&gfx, &mut model);

    let mut last_frame_time = Instant::now();

    while !window.should_close() {
        let delta_time = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();
        let time = glfw.get_time();

        window.swap_buffers();
        glfw.poll_events();
        process_input(&mut window, &events, &mut state, &cfg, delta_time);
        draw(&gfx, &mut state, time as f32, &mut model);
    }
    unsafe {
        gl::DeleteVertexArrays(
            gfx.vertex_array_objects.len() as i32,
            gfx.vertex_array_objects.as_ptr(),
        );
        gl::DeleteBuffers(gfx.array_buffers.len() as i32, gfx.array_buffers.as_ptr());
        for id in gfx.shader_programs {
            gl::DeleteProgram(id)
        }
        model.free_gl_resources();
    }
}
