use crate::gl;
use crate::state_and_cfg::GlData;
use mat_vec::Matrix4x4;

pub unsafe fn draw_face_culling(
    gfx: &GlData,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
) {
    gl::UseProgram(gfx.shader_programs[7]);
    gfx.set_uniform_mat4x4("model_mat", 7, &Matrix4x4::identity_matrix());
    gfx.set_uniform_mat4x4("view_mat", 7, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 7, projection_matrix);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}

#[allow(unused_imports)]
pub unsafe fn setup_face_culling() {
    gl::Enable(gl::CULL_FACE);
    use gl::{BACK, FRONT, FRONT_AND_BACK};
    gl::CullFace(FRONT);
    use gl::{CCW /* default */, CW};
    gl::FrontFace(CCW);
}
