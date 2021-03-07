use crate::gl;
use crate::state_and_cfg::GlData;
use ::opengl_learn::Model;
use matrix::Matrix4x4;

#[derive(Copy, Clone)]
#[allow(unused)]
pub enum GeomShdUseOpt {
    Houses,
    ExplodeEffect,
    DrawNormals,
}
pub use GeomShdUseOpt::*;

pub unsafe fn draw_geometry_shd_use(
    gfx: &GlData,
    model: &mut Model,
    time: f32,
    opt: GeomShdUseOpt,
) {
    if let Houses = opt {
        gl::BindVertexArray(gfx.vertex_array_objects[4]);
        gl::DrawArrays(gl::POINTS, 0, 4);
    }
    let model_mat = Matrix4x4::identity_matrix();
    if let ExplodeEffect = opt {
        let shd_idx = gfx.get_shader_program_index("Explode Effect shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        gfx.set_uniform_1f("time", shd_idx, time);
        model.draw();
    }
    if let DrawNormals = opt {
        let shd_idx = gfx.get_shader_program_index("UB Default shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        model.draw();

        let shd_idx = gfx.get_shader_program_index("Draw Normals shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        gfx.set_uniform_4f("rgba", shd_idx, 1.0, 0.5, 0.0, 0.3);
        model.draw();
    }
}

pub unsafe fn setup_geometry_shd_use(gfx: &GlData, model: &mut Model, opt: GeomShdUseOpt) {
    let shd_idx = match opt {
        Houses => gfx.get_shader_program_index("Geometry Shader Use 1"),
        ExplodeEffect => gfx.get_shader_program_index("Explode Effect shader"),
        DrawNormals => gfx.get_shader_program_index("Draw Normals shader"),
    };
    let shd_id = gfx.shader_programs[shd_idx];
    let uniform_block_idx = gl::GetUniformBlockIndex(
        shd_id,
        "Matrices\0".as_ptr() as *const i8, /* Rustfmt force vertical formatting */
    );
    gl::UniformBlockBinding(shd_id, uniform_block_idx, 0);

    match opt {
        Houses => {
            gl::UseProgram(gfx.shader_programs[shd_idx]);
            let model_mat = Matrix4x4::identity_matrix();
            gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        }
        ExplodeEffect | DrawNormals => {
            model.load_model("assets/backpack/", "backpack.obj");
            model.load_textures_to_gl(
                gl::CLAMP_TO_BORDER,
                gl::CLAMP_TO_BORDER,
                gl::LINEAR_MIPMAP_LINEAR,
                gl::LINEAR,
            );
            model.setup_draw();
        }
    }
    if let DrawNormals = opt {
        let shd_idx = gfx.get_shader_program_index("UB Default shader");
        let shd_id = gfx.shader_programs[shd_idx];
        let uniform_block_idx = gl::GetUniformBlockIndex(
            shd_id,
            "Matrices\0".as_ptr() as *const i8, /* Rustfmt force vertical formatting */
        );
        gl::UniformBlockBinding(shd_id, uniform_block_idx, 0);
    }
    gl::Enable(gl::BLEND);
    gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
}
