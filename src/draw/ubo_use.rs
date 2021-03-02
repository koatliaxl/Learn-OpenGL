use crate::gl;
use crate::state_and_cfg::GlData;
use matrix::Matrix4x4;
use std::ffi::c_void;

pub unsafe fn draw_ubo_use(gfx: &GlData) {
    gl::BindVertexArray(gfx.vertex_array_objects[3]);

    let shd_idx = gfx.get_shader_program_index("UBO Use shader 1");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let model_mat = Matrix4x4::new_translation(-1.5, -1.5, 0.0);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gl::DrawElements(
        gl::TRIANGLES, /* Rustfmt force vertical formatting */
        36,
        gl::UNSIGNED_INT,
        0 as *const c_void,
    );

    let shd_idx = gfx.get_shader_program_index("UBO Use shader 2");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let model_mat = Matrix4x4::new_translation(1.5, -1.5, 0.0);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gl::DrawElements(
        gl::TRIANGLES, /* Rustfmt force vertical formatting */
        36,
        gl::UNSIGNED_INT,
        0 as *const c_void,
    );

    let shd_idx = gfx.get_shader_program_index("UBO Use shader 3");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let model_mat = Matrix4x4::new_translation(0.0, 1.5, 0.0);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gl::DrawElements(
        gl::TRIANGLES, /* Rustfmt force vertical formatting */
        36,
        gl::UNSIGNED_INT,
        0 as *const c_void,
    );
}

pub unsafe fn setup_ubo_use(gfx: &GlData) {
    let shd_id = gfx.get_shader_program_gl_id("UBO Use shader 1");
    let uniform_block_idx = gl::GetUniformBlockIndex(
        shd_id,
        "Matrices\0".as_ptr() as *const i8, /* Rustfmt force vertical formatting */
    );
    gl::UniformBlockBinding(shd_id, uniform_block_idx, 0);

    let shd_id = gfx.get_shader_program_gl_id("UBO Use shader 2");
    let uniform_block_idx = gl::GetUniformBlockIndex(
        shd_id,
        "Matrices\0".as_ptr() as *const i8, /* Rustfmt force vertical formatting */
    );
    gl::UniformBlockBinding(shd_id, uniform_block_idx, 0);

    let shd_id = gfx.get_shader_program_gl_id("UBO Use shader 3");
    let uniform_block_idx = gl::GetUniformBlockIndex(
        shd_id,
        "Matrices\0".as_ptr() as *const i8, /* Rustfmt force vertical formatting */
    );
    gl::UniformBlockBinding(shd_id, uniform_block_idx, 0);
}
