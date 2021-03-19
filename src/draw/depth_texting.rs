use super::{draw_floor, draw_two_containers};
use crate::gl;
use crate::state_and_cfg::GlData;
use mat_vec::Matrix4x4;

#[derive(Copy, Clone)]
#[allow(unused)]
pub enum VisualisationMode {
    PlainScene,
    LinearizedDepthValue,
    DepthValue,
}
pub use VisualisationMode::*;

pub unsafe fn draw_depth_testing_scene(
    gfx: &GlData,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    mode: VisualisationMode,
) {
    gl::UseProgram(gfx.shader_programs[5]);
    gfx.set_uniform_mat4x4("view_mat", 5, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 5, projection_matrix);
    gfx.set_uniform_1i("depth_visualization_mode", 5, mode.int_code());
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);

    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[7]);
    draw_two_containers(gfx, 5, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 5);
}

pub unsafe fn setup_depth_testing_scene() {
    gl::DepthMask(gl::TRUE);
    #[allow(unused_imports)]
    use crate::gl::{ALWAYS, EQUAL, GEQUAL, GREATER, LEQUAL, LESS, NEVER, NOTEQUAL};
    gl::DepthFunc(LESS);
}

impl VisualisationMode {
    fn int_code(&self) -> i32 {
        match self {
            VisualisationMode::PlainScene => 0,
            VisualisationMode::LinearizedDepthValue => 1,
            VisualisationMode::DepthValue => 2,
        }
    }
}
