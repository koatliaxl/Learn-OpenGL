use crate::gl;
use crate::state_and_cfg::GlData;
use mat_vec::Matrix4x4;
use std::ffi::c_void;

pub unsafe fn draw_antialiasing(_gfx: &GlData) {
    gl::DrawElements(
        gl::TRIANGLES, /* Rustfmt force vertical formatting */
        36,
        gl::UNSIGNED_INT,
        0 as *const c_void,
    );
}

pub unsafe fn setup_antialiasing(gfx: &mut GlData) {
    gl::Enable(gl::MULTISAMPLE);

    let shd_idx = gfx.get_shader_program_index("UBO Use shader 2");
    //let shd_idx = gfx.get_shader_program_index("UB Default shader");

    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let model_mat = Matrix4x4::identity_matrix();
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);

    gl::BindVertexArray(gfx.vertex_array_objects[1]);
    /*gl::BindVertexArray(gfx.vertex_array_objects[3]);
    gl::ActiveTexture(gl::TEXTURE0);
    let tex_id = gfx.get_texture_gl_id("Container texture");
    gl::BindTexture(gl::TEXTURE_2D, tex_id);*/
}
