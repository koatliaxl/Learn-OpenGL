use crate::draw::IDENTITY_MATRIX;
use crate::gl;
use crate::state_and_cfg::{GlData, State};
use mat_vec::{Matrix4x4, Vector3};

pub unsafe fn draw_normal_mapping(gfx: &GlData, state: &State) {
    let shd_idx = gfx.get_shader_program_index("Advanced Lighting shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_vec3f("Viewer_Position", shd_idx, state.camera.position);
    gfx.set_uniform_1b("normal_mapping", shd_idx, state.normal_mapping);
    gfx.set_uniform_1f("Shininess", shd_idx, state.shininess);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);

    let shd_id = gfx.get_shader_program_gl_id("Single Color shader");
    gl::UseProgram(shd_id);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}

pub unsafe fn setup_normal_mapping(gfx: &GlData) {
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.get_texture_gl_id("Brick wall"));
    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_2D, gfx.get_texture_gl_id("Brick wall normal"));

    let light_source_pos = Vector3::new(1.0, 1.0, 2.0);

    let shd_idx = gfx.get_shader_program_index("Advanced Lighting shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_1i("Light_Sources_Num", shd_idx, 1);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &IDENTITY_MATRIX);
    gfx.set_uniform_1i("normal_map", shd_idx, 1);
    gfx.set_uniform_vec3f("Light_Sources[0].position", shd_idx, light_source_pos);
    gfx.set_uniform_3f("Light_Sources[0].color", shd_idx, 1.0, 1.0, 1.0);
    gfx.set_uniform_1f("attenuation_linear_term", shd_idx, 0.1);
    gfx.set_uniform_1f("attenuation_quadratic_term", shd_idx, 0.03);
    gfx.set_uniform_1u("normal_mapping", shd_idx, 1);
    //gfx.set_uniform_1f("specular_factor", shd_idx, 0.4);

    let shd_idx = gfx.get_shader_program_index("Single Color shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let scaling = Matrix4x4::new_uniform_scaling(0.1);
    //let (lx, ly, lz) = light_source_pos.get_components();
    let translation = Matrix4x4::new_translation_from_vec(light_source_pos);
    let model_mat = translation * scaling;
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gfx.set_uniform_vec3f("color", shd_idx, Vector3::new(1.0, 1.0, 1.0));
}
