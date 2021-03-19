use super::{draw_floor, draw_two_containers};
use crate::gl;
use crate::state_and_cfg::{GlData, State};
use mat_vec::{Matrix4x4, Vector3};
use std::cmp::Ordering::Equal;

static POSITIONS: [(f32, f32, f32); 5] = [
    (-1.5, 0.0, -0.48),
    (1.5, 0.0, 0.51),
    (0.0, 0.0, 0.7),
    (-0.3, 0.0, -2.3),
    (0.5, 0.0, -0.6),
];

pub unsafe fn draw_blending_scene(
    gfx: &GlData,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    state: &State,
) {
    gl::UseProgram(gfx.shader_programs[7]);
    gfx.set_uniform_mat4x4("view_mat", 7, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 7, projection_matrix);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);

    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[7]);
    draw_two_containers(gfx, 7, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 7);

    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[8]);
    let mut transparent_windows_positions = POSITIONS
        .iter()
        .map(|(x, y, z)| {
            let pos = Vector3::new(*x, *y, *z);
            let dist_vec = state.camera.position - pos;
            let distance = dist_vec.length();
            (distance, pos)
        })
        .collect::<Vec<(f32, Vector3<f32>)>>();
    //transparent_windows_positions.sort_by_key(|(distance, _)| *distance);
    transparent_windows_positions.sort_by(|(dist_1, _), (dist_2, _)| {
        if let Some(ord) = dist_2.partial_cmp(dist_1) {
            ord
        } else {
            Equal
        }
    });
    for (_, pos) in transparent_windows_positions {
        let (x, y, z) = pos.get_components();
        let model_mat = Matrix4x4::new_translation(x + 0.5, y, z + 0.5);
        gfx.set_uniform_mat4x4("model_mat", 7, &model_mat);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
}

#[allow(unused_imports)]
pub unsafe fn setup_blending_scene() {
    gl::Enable(gl::BLEND);
    use gl::{CONSTANT_ALPHA, CONSTANT_COLOR, ONE_MINUS_CONSTANT_ALPHA, ONE_MINUS_CONSTANT_COLOR};
    use gl::{DST_ALPHA, DST_COLOR, ONE_MINUS_DST_ALPHA, ONE_MINUS_DST_COLOR};
    use gl::{ONE, ZERO};
    use gl::{ONE_MINUS_SRC_ALPHA, ONE_MINUS_SRC_COLOR, SRC_ALPHA, SRC_COLOR};
    gl::BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    //gl::BlendFuncSeparate(SRC_ALPHA, ONE_MINUS_SRC_ALPHA, ONE, ZERO);
    //gl::BlendColor(1.0, 1.0, 1.0, 1.0); // sets the Constant Color
    use gl::{
        FUNC_ADD,              // the default
        FUNC_REVERSE_SUBTRACT, // subtracts both colors, but reverses order
        FUNC_SUBTRACT,
        MAX, // takes the component-wise maximum of both colors
        MIN, // takes the component-wise minimum of both colors
    };
    //gl::BlendEquation(FUNC_ADD);
}
