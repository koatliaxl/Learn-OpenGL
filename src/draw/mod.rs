mod backpack;
mod blending;
mod cubes;
mod depth_texting;
mod lighting;
mod stencil_testing;

use self::cubes::draw_cubes;
use self::{backpack::*, blending::*, depth_texting::*, lighting::*, stencil_testing::*};
use crate::gl;
use crate::state_and_cfg::{GlData, State};
use ::opengl_learn::Model;
use matrix::Matrix4x4;
use Draw::*;

static DRAW: Draw = BlendingScene;

#[allow(unused)]
enum Draw {
    Triangle,
    Cubes,
    LightingScene,
    Backpack,
    DepthTestingScene,
    StencilTestingScene,
    BlendingScene,
    TextureMinFilterTest,
}

pub fn draw(gfx: &GlData, state: &mut State, time: f32, model: &mut Model) {
    unsafe {
        state.camera.recalculate_look_at_matrix();
        let view_mat = state.camera.look_at_matrix.clone();
        let projection_mat = Matrix4x4::new_perspective_projection(
            state.field_of_view, /* Rustfmt force vertical formatting */
            state.aspect_ratio,
            100.0,
            0.1,
        );

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        match DRAW {
            Triangle => draw_triangle(gfx, time),
            Cubes => {
                draw_cubes(gfx, state, &view_mat, &projection_mat, time);
            }
            LightingScene => draw_lighting_scene(
                &gfx,
                &view_mat,
                &projection_mat,
                time,
                state, /* Rustfmt force vertical formatting */
            ),
            Backpack => draw_backpack(
                &gfx,
                model,
                &view_mat,
                &projection_mat,
                state, /* Rustfmt force vertical formatting */
            ),
            DepthTestingScene => draw_depth_testing_scene(
                &gfx,
                &view_mat,
                &projection_mat,
                1, /* Rustfmt force vertical formatting */
            ),
            StencilTestingScene => draw_stencil_testing_scene(&gfx, &view_mat, &projection_mat),
            BlendingScene => draw_blending_scene(&gfx, &view_mat, &projection_mat, &state),
            _ => {}
        }
    }
}

pub fn init_draw(gfx: &GlData, model: &mut Model) {
    unsafe {
        /*gl::UseProgram(gfx.shader_programs[1]);
        gl::Uniform1f(gfx.get_var_loc("Zoom", 1), 1.0);*/
        gl::Enable(gl::DEPTH_TEST);
        match DRAW {
            Triangle | Cubes => gl::ClearColor(0.2, 0.2, 0.7, 1.0),
            LightingScene => init_lighting_scene(&gfx),
            Backpack => setup_backpack_draw(gfx, model),
            DepthTestingScene => setup_depth_testing_scene(),
            StencilTestingScene => setup_stencil_testing_scene(),
            BlendingScene => setup_blending_scene(),
            _ => {}
        }
    }
}

unsafe fn draw_floor(gfx: &GlData, shader_program_idx: usize) {
    let mut model_mat = Matrix4x4::new_scaling(10.0, 10.0, 0.0);
    model_mat = Matrix4x4::new_x_rotation(90.0) * model_mat;
    model_mat = Matrix4x4::new_translation(0.0, -0.5, 0.0) * model_mat;
    gfx.set_uniform_mat4x4("model_mat", shader_program_idx, &model_mat);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
}

unsafe fn draw_two_containers(gfx: &GlData, shader_program_idx: usize, scale: f32) {
    let scaling_mat = Matrix4x4::new_scaling(scale, scale, scale);
    let mut model_mat;
    model_mat = Matrix4x4::new_translation(-1.0, 0.001, -1.0) * &scaling_mat;
    gfx.set_uniform_mat4x4("model_mat", shader_program_idx, &model_mat);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
    model_mat = Matrix4x4::new_translation(2.0, 0.001, 0.0) * scaling_mat;
    gfx.set_uniform_mat4x4("model_mat", shader_program_idx, &model_mat);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}

#[allow(dead_code)]
unsafe fn draw_triangle(gfx: &GlData, time: f32) {
    let time_sine = (time % 360.0).sin();
    gl::UseProgram(gfx.shader_programs[0]);
    gl::Uniform1f(gfx.get_var_loc("offset", 0), time_sine);
    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);

    gl::BindVertexArray(gfx.vertex_array_objects[0]);
    gl::DrawArrays(gl::TRIANGLES, 0, 3);
}
