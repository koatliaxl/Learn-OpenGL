use crate::gl;
#[allow(unused_imports)]
use crate::gl::{ALWAYS, EQUAL, GEQUAL, GREATER, LEQUAL, LESS, NEVER, NOTEQUAL};
use crate::state_and_cfg::GlData;
use matrix::Matrix4x4;
#[allow(unused_imports)]
use opengl_learn::gl::{FALSE, TRUE};

pub unsafe fn draw_depth_or_stencil_testing_scene(
    gfx: &GlData,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    draw_outline: bool,
    depth_visualization_mode: i32,
) {
    gl::UseProgram(gfx.shader_programs[5]);
    gfx.set_uniform_mat4x4("view_mat", 5, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 5, projection_matrix);
    gfx.set_uniform_1i("depth_visualization_mode", 5, depth_visualization_mode);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);

    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    let mut model_mat = Matrix4x4::new_scaling(10.0, 10.0, 0.0);
    model_mat = Matrix4x4::new_x_rotation(90.0) * model_mat;
    model_mat = Matrix4x4::new_translation(0.0, -0.5, 0.0) * model_mat;
    gfx.set_uniform_mat4x4("model_mat", 5, &model_mat);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    if draw_outline {
        gl::Clear(gl::STENCIL_BUFFER_BIT);
        gl::Enable(gl::STENCIL_TEST);
        // all fragments should pass the stencil test
        gl::StencilFunc(ALWAYS, 1, 0xFFFF_FFFF);
    }
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[7]);
    draw_two_containers(gfx, 5, 1.0);

    if draw_outline {
        gl::StencilFunc(NOTEQUAL, 1, 0xFFFF_FFFF);
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
        gl::StencilMask(0xFFFF_FFFF);
    }
}

unsafe fn draw_two_containers(gfx: &GlData, shader_program_idx: usize, scale: f32) {
    let scaling_mat = Matrix4x4::new_scaling(scale, scale, scale);
    let mut model_mat;
    model_mat = Matrix4x4::new_translation(-1.0, 0.001, 1.0) * &scaling_mat;
    gfx.set_uniform_mat4x4("model_mat", shader_program_idx, &model_mat);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
    model_mat = Matrix4x4::new_translation(2.0, 0.001, 0.0) * scaling_mat;
    gfx.set_uniform_mat4x4("model_mat", shader_program_idx, &model_mat);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}

pub unsafe fn setup_depth_testing_scene() {
    gl::DepthMask(TRUE);
    gl::DepthFunc(LESS);
}

pub unsafe fn setup_stencil_testing_scene() {
    //gl::Enable(STENCIL_TEST);
    #[allow(unused_imports)]
    use crate::gl::{DECR, DECR_WRAP, INCR, INCR_WRAP, INVERT, KEEP, REPLACE, ZERO};
    gl::StencilOp(KEEP, KEEP, REPLACE);
}
