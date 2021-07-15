use super::{draw_floor, draw_two_containers};
use crate::gl;
#[allow(unused_imports)]
use crate::gl::{ALWAYS, EQUAL, GEQUAL, GREATER, LEQUAL, LESS, NEVER, NOTEQUAL};
use crate::state_and_cfg::GlData;
use mat_vec::Matrix4x4;

pub unsafe fn draw_stencil_testing_scene(
    gfx: &GlData,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
) {
    gl::UseProgram(gfx.shader_programs[5]);
    gfx.set_uniform_mat4x4("view_mat", 5, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 5, projection_matrix);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);

    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 5, 10.0);

    gl::Clear(gl::STENCIL_BUFFER_BIT);
    gl::Enable(gl::STENCIL_TEST);
    // all fragments should pass the stencil test
    gl::StencilFunc(ALWAYS, 1, 0xFF);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[7]);
    draw_two_containers(gfx, 5, 1.0);

    gl::StencilFunc(NOTEQUAL, 1, 0xFF);
    gl::StencilMask(0); // each bit ends up as 0 in the stencil buffer (disabling writes)
    gl::Disable(gl::DEPTH_TEST);
    gl::UseProgram(gfx.shader_programs[6]);
    gfx.set_uniform_mat4x4("view_mat", 6, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 6, projection_matrix);
    gfx.set_uniform_3f("color", 6, 0.04, 0.28, 0.26);
    draw_two_containers(gfx, 6, 1.1);
    gl::Disable(gl::STENCIL_TEST);
    gl::Enable(gl::DEPTH_TEST);
    /* each bit is written to the stencil buffer as is (enable writing);
    also affects gl::Clear, therefore must have right value before call of it ! */
    gl::StencilMask(0xFF);
}

pub unsafe fn setup_stencil_testing_scene() {
    //gl::Enable(STENCIL_TEST);
    #[allow(unused_imports)]
    use crate::gl::{DECR, DECR_WRAP, INCR, INCR_WRAP, INVERT, KEEP, REPLACE, ZERO};
    gl::StencilOp(KEEP, KEEP, REPLACE);
}
