use crate::gl;
use crate::state_and_cfg::{GlData, State};
use matrix::{Matrix4x4, Vector3};
use opengl_learn::Model;

pub unsafe fn draw_backpack(
    gfx: &GlData,
    model: &mut Model,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    state: &State,
) {
    gl::UseProgram(gfx.shader_programs[4]);
    gfx.set_uniform_mat4x4("view_mat", 4, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 4, projection_matrix);
    gfx.set_uniform_vec3f("viewer_position", 4, state.camera.position);
    model.draw();

    gl::UseProgram(gfx.shader_programs[3]);
    gfx.set_uniform_mat4x4("view_mat", 3, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 3, projection_matrix);
    gfx.set_uniform_3f("color", 3, 1.0, 1.0, 1.0);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}

pub unsafe fn setup_backpack_draw(gfx: &GlData, model: &mut Model) {
    gl::ClearColor(0.0, 0.0, 0.0, 1.0);
    model.load_model("assets/backpack/", "backpack.obj");
    model.load_textures_to_gl();
    model.setup_draw();
    let ls_pos = Vector3::new(1.0, 2.0, 2.0);

    gl::UseProgram(gfx.shader_programs[4]);
    gfx.set_uniform_vec3f("point_lights[0].position", 4, ls_pos);
    gfx.set_uniform_1f("shininess", 4, 32.0);
    gfx.set_uniform_1u("diffuse_texture", 4, 0);
    gfx.set_uniform_1u("specular_map", 4, 1);
    gfx.set_uniform_mat4x4("model_mat", 4, &Matrix4x4::identity_matrix());

    gl::UseProgram(gfx.shader_programs[3]);
    let ls_scaling_mat = Matrix4x4::new_scaling(0.1, 0.1, 0.1);
    let (lx, ly, lz) = ls_pos.get_components();
    let ls_translation_mat = Matrix4x4::new_translation(lx, ly, lz);
    let ls_model_matrix = ls_translation_mat * ls_scaling_mat;
    gfx.set_uniform_mat4x4("model_mat", 3, &ls_model_matrix);
}
