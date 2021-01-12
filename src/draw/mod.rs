mod backpack;
mod cubes;
mod depth_texting;
mod lighting;

use crate::draw::depth_texting::*;
use crate::gl;
use crate::state_and_cfg::{GlData, State};
use ::opengl_learn::Model;
use backpack::*;
use cubes::draw_cubes;
use lighting::*;
use matrix::Matrix4x4;
use Draw::*;

static DRAW: Draw = StencilTestingScene;

#[allow(unused)]
enum Draw {
    Triangle,
    Cubes,
    LightingScene,
    Backpack,
    DepthTestingScene,
    StencilTestingScene,
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
            DepthTestingScene => draw_depth_or_stencil_testing_scene(
                &gfx,
                &view_mat,
                &projection_mat,
                false, /* Rustfmt force vertical formatting */
                1,
            ),
            StencilTestingScene => draw_depth_or_stencil_testing_scene(
                &gfx,
                &view_mat,
                &projection_mat,
                true, /* Rustfmt force vertical formatting */
                0,
            ),
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
            _ => {}
        }
    }
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
