use crate::gl;
use crate::state_and_cfg::GlData;
use mat_vec::{Matrix4x4, Vector3};

pub unsafe fn draw_adv_lighting(
    gfx: &GlData,
    camera_pos: Vector3<f32>,
    blinn_phong_lighting: bool,
) {
    let shd_idx = gfx.get_shader_program_index("Advanced Lighting shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_vec3f("Viewer_Position", shd_idx, camera_pos);
    let (biv, shininess) = match blinn_phong_lighting {
        true => (1, 32.0),
        false => (0, 8.0),
    };
    gfx.set_uniform_1u("Blinn_Phong_Lighting", shd_idx, biv);
    gfx.set_uniform_1f("Shininess", shd_idx, shininess);
    gl::DrawArrays(gl::TRIANGLES, 12, 6);

    gfx.use_shader_program("Single Color shader");
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}

pub unsafe fn setup_adv_lighting(gfx: &GlData) {
    let tex_id = gfx.get_texture_gl_id("Wood Flooring");
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, tex_id);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);

    let ls_pos = Vector3::new(3.0, 3.0, 0.0);
    let ls_color = Vector3::new(1.0, 1.0, 1.0);

    let shd_idx = gfx.get_shader_program_index("Advanced Lighting shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let model_mat = Matrix4x4::new_scaling(20.0, 1.0, 20.0);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gfx.set_uniform_1i("Light_Sources_Num", shd_idx, 1);
    gfx.set_uniform_vec3f("Light_Sources[0].position", shd_idx, ls_pos);
    gfx.set_uniform_vec3f("Light_Sources[0].color", shd_idx, ls_color);
    gfx.set_uniform_1f("Shininess", shd_idx, 8.0);

    let shd_idx = gfx.get_shader_program_index("Single Color shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let ls_model_mat = Matrix4x4::new_translation(ls_pos.x(), ls_pos.y(), ls_pos.z());
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &ls_model_mat);
    gfx.set_uniform_vec3f("color", shd_idx, ls_color);
}
