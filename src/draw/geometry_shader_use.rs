use crate::gl;
use crate::state_and_cfg::GlData;
//use ::opengl_learn::{SIZE_OF_GL_FLOAT, SIZE_OF_GL_UNSIGNED_INT};
use matrix::Matrix4x4;
//use std::ffi::c_void;

pub unsafe fn draw_geometry_shd_use(gfx: &GlData) {
    gl::BindVertexArray(gfx.vertex_array_objects[4]);
    /*gl::DrawArrays(gl::POINTS, 0, 3);
    gl::DrawArrays(gl::POINTS, 4, 1);*/
    gl::DrawArrays(gl::POINTS, 0, 4);
    /* gl::DrawElements(
        gl::POINTS, /* Rustfmt force vertical formatting */
        3,
        gl::UNSIGNED_INT,
        0 as *const c_void,
    );
    gl::DrawElements(
        gl::POINTS, /* Rustfmt force vertical formatting */
        1,
        gl::UNSIGNED_INT,
        (4 * SIZE_OF_GL_UNSIGNED_INT) as *const c_void,
    );*/
}

pub unsafe fn setup_geometry_shd_use(gfx: &GlData) {
    let shd_idx = gfx.get_shader_program_index("Geometry Shader Use 1");
    let shd_id = gfx.shader_programs[shd_idx];
    let uniform_block_idx = gl::GetUniformBlockIndex(
        shd_id,
        "Matrices\0".as_ptr() as *const i8, /* Rustfmt force vertical formatting */
    );
    gl::UniformBlockBinding(shd_id, uniform_block_idx, 0);

    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let model_mat = Matrix4x4::identity_matrix();
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);

    //gl::PointSize(5.0);

    /*gl::BindVertexArray(gfx.vertex_array_objects[1]);
    gl::VertexAttribPointer(
        3,
        3,
        gl::FLOAT,
        gl::FALSE,
        ((3 + 3 + 2) * SIZE_OF_GL_FLOAT) as i32,
        (SIZE_OF_GL_FLOAT as i32 * 3) as *const c_void,
    );
    gl::EnableVertexAttribArray(3);*/
}
