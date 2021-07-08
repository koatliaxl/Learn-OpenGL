use crate::gl;
use crate::state_and_cfg::{GlData, State};
use mat_vec::{Matrix4x4, Vector3};
use GammaCorrection::*;

static mut LIGHT_POSITIONS: Vec<Vector3<f32>> = Vec::new();
static mut LIGHT_COLOR: Vec<Vector3<f32>> = Vec::new();
static LIGHT_SOURCES_NUM: usize = 4;

#[derive(Copy, Clone)]
pub enum GammaCorrection {
    Disabled,
    BuiltinOpenGL,
    InShader,
}

pub unsafe fn draw_gamma_correction(gfx: &GlData, state: &State) {
    let shd_idx = gfx.get_shader_program_index("Advanced Lighting shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_vec3f("Viewer_Position", shd_idx, state.camera.position);
    match state.gamma_correction {
        Disabled => {
            //gl::Disable(gl::FRAMEBUFFER_SRGB); // no needed, because toggles in a specific order
            gfx.set_uniform_1u("Gamma_Correction", shd_idx, 0);
        }
        BuiltinOpenGL => {
            gl::Enable(gl::FRAMEBUFFER_SRGB);
            //gfx.set_uniform_1u("Gamma_Correction", shd_idx, 0);
            // no needed, because toggles in a specific order
        }
        InShader => {
            gl::Disable(gl::FRAMEBUFFER_SRGB);
            gfx.set_uniform_1u("Gamma_Correction", shd_idx, 1);
        }
    }
    gfx.set_uniform_1f("Shininess", shd_idx, state.shininess);
    let tex_id = if state.srgb_texture {
        gfx.get_texture_gl_id("Wood Flooring sRGB")
    } else {
        gfx.get_texture_gl_id("Wood Flooring")
    };
    gl::BindTexture(gl::TEXTURE_2D, tex_id);
    gfx.set_uniform_1f(
        "attenuation_constant_term",
        shd_idx,
        state.attenuation.constant_term,
    );
    gfx.set_uniform_1f(
        "attenuation_linear_term",
        shd_idx,
        state.attenuation.linear_term,
    );
    gfx.set_uniform_1f(
        "attenuation_quadratic_term",
        shd_idx,
        state.attenuation.quadratic_term,
    );

    gl::DrawArrays(gl::TRIANGLES, 12, 6);

    let shd_idx = gfx.get_shader_program_index("Single Color shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    for i in 0..LIGHT_SOURCES_NUM {
        let (x, y, z) = LIGHT_POSITIONS[i].get_components();
        let scaling = Matrix4x4::new_scaling(0.1, 0.1, 0.1);
        let model_mat = Matrix4x4::new_translation(x, y, z) * scaling;
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        gfx.set_uniform_vec3f("color", shd_idx, LIGHT_COLOR[i]);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
    }
}

pub unsafe fn setup_gamma_correction(gfx: &mut GlData, state: &mut State) {
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    state.shininess = 64.0;

    use crate::gl::{LINEAR, LINEAR_MIPMAP_LINEAR, REPEAT};
    use crate::load_tex::load_as_srgb;
    let srgb_texture = load_as_srgb(
        "assets/wood.png",
        REPEAT,
        REPEAT,
        LINEAR_MIPMAP_LINEAR,
        LINEAR,
    );
    gfx.add_texture(srgb_texture, "Wood Flooring sRGB");

    LIGHT_POSITIONS.push(Vector3::new(-6.0, 1.0, 0.0));
    LIGHT_POSITIONS.push(Vector3::new(-2.0, 1.0, 0.0));
    LIGHT_POSITIONS.push(Vector3::new(2.0, 1.0, 0.0));
    LIGHT_POSITIONS.push(Vector3::new(6.0, 1.0, 0.0));
    for i in 0..LIGHT_SOURCES_NUM {
        LIGHT_COLOR.push(Vector3::new(0.25, 0.25, 0.25_f32) * (i + 1) as u32);
    }

    let shd_idx = gfx.get_shader_program_index("Advanced Lighting shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let floor_model_mat = Matrix4x4::new_scaling(20.0, 1.0, 20.0);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &floor_model_mat);
    gfx.set_uniform_1i("Light_Sources_Num", shd_idx, LIGHT_SOURCES_NUM as i32);
    for i in 0..LIGHT_SOURCES_NUM {
        let var_pos = format!("Light_Sources[{}].position", i);
        let var_color = format!("Light_Sources[{}].color", i);
        gfx.set_uniform_vec3f(&var_pos, shd_idx, LIGHT_POSITIONS[i]);
        gfx.set_uniform_vec3f(&var_color, shd_idx, LIGHT_COLOR[i]);
    }
    gfx.set_uniform_1f("ambient_strength", shd_idx, 0.0);
    //gfx.set_uniform_1f("specular_coef", shd_idx, 1.0);
}
