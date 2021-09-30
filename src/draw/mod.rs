mod adv_data_use;
mod adv_lighting;
mod antialiasing;
mod backpack;
mod blending;
mod cubemap;
mod cubes;
mod depth_texting;
mod face_culling;
mod framebuffers;
mod geometry_shader_use;
mod instancing;
mod lighting;
mod stencil_testing;
mod ubo_use;

pub use adv_lighting::{
    Attenuation, GammaCorrection, LightProjectionMatrix, NormalMappingSettings,
    OmnidirectionalShadowMappingSetting, ShadowMappingSettings,
};
pub use framebuffers::PostProcessingOption;

use self::cubes::draw_cubes;
use self::{
    adv_data_use::*, adv_lighting::*, antialiasing::*, backpack::*, blending::*, cubemap::*,
    depth_texting::*, face_culling::*, framebuffers::*, geometry_shader_use::*, instancing::*,
    lighting::*, stencil_testing::*, ubo_use::*,
};
use crate::gl;
use crate::state_and_cfg::{GlData, State};
use ::opengl_learn::Model;
use glfw::Window;
use mat_vec::Matrix4x4;
use std::ffi::c_void;
use Draw::*;

pub const IDENTITY_MATRIX: Matrix4x4<f32> = Matrix4x4::<f32>::IDENTITY_MATRIX;

pub static DRAW: Draw = GeometryShaderUse(Houses);

#[allow(unused)]
pub enum Draw {
    Triangle,
    Cubes,
    LightingScene,
    Backpack,
    DepthTestingScene(VisualisationMode),
    StencilTestingScene,
    BlendingScene,
    FaceCulling,
    FrameBuffers,
    CubeMap,
    UniformBufferObjectsUse,
    GeometryShaderUse(GeomShdUseOpt),
    Instancing(InstancingOption),
    AntiAliasing, /*{ samples: u32 }*/

    BlinnPhongLighting,
    GammaCorrection,
    ShadowMapping,
    PointShadows,
    NormalMapping,

    _AdvDataUse,
    TextureMinFilterTest,
}

pub fn draw(gfx: &GlData, state: &mut State, time: f32, model: &mut Model, window: &Window) {
    unsafe {
        state.camera.recalculate_look_at_matrix();
        let view_mat = state.camera.look_at_matrix.clone();
        let projection_mat = Matrix4x4::new_perspective_projection(
            state.field_of_view, /* Rustfmt force vertical formatting */
            state.aspect_ratio,
            100.0,
            0.1,
        );
        let mat_size = view_mat.size_of_raw_value();
        let buf_gl_id = gfx.get_uniform_buffer_gl_id("Matrices");
        gl::BindBuffer(gl::UNIFORM_BUFFER, buf_gl_id);
        gl::BufferSubData(
            gl::UNIFORM_BUFFER,
            0,
            mat_size as isize,
            view_mat.transpose().as_ptr() as *const c_void,
        );
        gl::BufferSubData(
            gl::UNIFORM_BUFFER,
            mat_size as isize,
            mat_size as isize,
            projection_mat.transpose().as_ptr() as *const c_void,
        );

        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        match DRAW {
            Triangle => draw_triangle(gfx, time),
            Cubes => {
                draw_cubes(gfx, state, &view_mat, &projection_mat, time);
            }
            LightingScene => draw_lighting_scene(
                gfx,
                &view_mat,
                &projection_mat,
                time,
                state, /* Rustfmt force vertical formatting */
            ),
            Backpack => draw_backpack(
                gfx,
                model,
                &view_mat,
                &projection_mat,
                state, /* Rustfmt force vertical formatting */
            ),
            DepthTestingScene(mode) => draw_depth_testing_scene(
                gfx,
                &view_mat,
                &projection_mat,
                mode, /* Rustfmt force vertical formatting */
            ),
            StencilTestingScene => draw_stencil_testing_scene(
                gfx,
                &view_mat,
                &projection_mat, /* Rustfmt force vertical formatting */
            ),
            BlendingScene => draw_blending_scene(
                gfx,
                &view_mat,
                &projection_mat,
                &state, /* Rustfmt force vertical formatting */
            ),
            FaceCulling => draw_face_culling(gfx, &view_mat, &projection_mat),
            FrameBuffers => draw_framebuffers(
                gfx,
                //&view_mat,
                &projection_mat,
                state,
            ),
            CubeMap => draw_cubemap_scene(
                gfx,
                &projection_mat,
                &state.camera,
                model,
                EnvironmentMappingMode::Refraction,
            ),
            UniformBufferObjectsUse => draw_ubo_use(gfx),
            GeometryShaderUse(opt) => draw_geometry_shd_use(gfx, model, time, opt),
            Instancing(_) => instancing_draw(gfx),
            AntiAliasing => draw_antialiasing(gfx),
            BlinnPhongLighting => {
                draw_blinn_phong_lighting(gfx, state.camera.position, state.blinn_phong_lighting)
            }
            GammaCorrection => draw_gamma_correction(gfx, state),
            ShadowMapping => draw_shadow_mapping(gfx, window, state),
            PointShadows => draw_point_shadows(gfx, window, state),
            NormalMapping => draw_normal_mapping(gfx, state),
            _ => {}
        }
    }
}

pub fn init_draw(gfx: &mut GlData, model: &mut Model, window: &Window, state: &mut State) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        match DRAW {
            Triangle | Cubes => gl::ClearColor(0.2, 0.2, 0.7, 1.0),
            LightingScene => init_lighting_scene(&gfx),
            Backpack => setup_backpack_draw(gfx, model),
            DepthTestingScene(_) => setup_depth_testing_scene(),
            StencilTestingScene => setup_stencil_testing_scene(),
            BlendingScene => setup_blending_scene(),
            FaceCulling => setup_face_culling(),
            FrameBuffers => setup_framebuffers(gfx, window.get_size()),
            CubeMap => setup_cubemap_scene(gfx, model),
            UniformBufferObjectsUse => setup_ubo_use(gfx),
            GeometryShaderUse(opt) => setup_geometry_shd_use(gfx, model, opt),
            Instancing(opt) => setup_instancing(gfx, opt, state),
            AntiAliasing => {
                setup_antialiasing(gfx, window.get_size(), 8, AntiAliasingMode::DirectOutput)
            }
            BlinnPhongLighting => setup_blinn_phong_lighting(gfx),
            GammaCorrection => setup_gamma_correction(gfx, state),
            ShadowMapping => setup_shadow_mapping(gfx, true),
            PointShadows => {
                setup_point_shadows(gfx);
                state.camera.speed = 25.0
            }
            NormalMapping => {
                setup_normal_mapping(gfx, true);
                state.shininess = 64.0;
                state.normal_mapping_settings.enabled = true;
            }

            _AdvDataUse => adv_data_use(gfx),
            _ => {}
        }
        let mut ubo_matrices = 0;
        gl::GenBuffers(1, &mut ubo_matrices);
        gl::BindBuffer(gl::UNIFORM_BUFFER, ubo_matrices);
        gl::BufferData(
            gl::UNIFORM_BUFFER,
            Matrix4x4::<f32>::size_of_raw_data() as isize * 2,
            std::ptr::null(),
            gl::STATIC_DRAW,
        );
        gl::BindBuffer(gl::UNIFORM_BUFFER, 0); // note that despite this, next work
        gl::BindBufferRange(
            gl::UNIFORM_BUFFER,
            0,
            ubo_matrices,
            0,
            Matrix4x4::<f32>::size_of_raw_data() as isize * 2,
        );
        // alternative to BindBufferRange()
        //gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, ubo_matrices);
        gfx.add_uniform_buffer(ubo_matrices, "Matrices");

        // Next not really needed because the "Matrices" binds to 0,
        // and uniform blocks of the shaders have this value initially.
        // But for convenience and in case...
        bind_uniform_block("Matrices", "UB Default shader", 0, gfx);
        bind_uniform_block("Matrices", "Instancing shader", 0, gfx);
        bind_uniform_block("Matrices", "UBO Use shader 2", 0, gfx);
        bind_uniform_block("Matrices", "Advanced Lighting shader", 0, gfx);
        bind_uniform_block("Matrices", "Single Color shader", 0, gfx);
        bind_uniform_block("Matrices", "Shadow Mapping shader", 0, gfx);
        bind_uniform_block("Matrices", "Depth cubemap shader", 0, gfx);
        bind_uniform_block("Matrices", "Point Shadows shader", 0, gfx);
        bind_uniform_block("Matrices", "Normal Mapping shader", 0, gfx);
    }
}

unsafe fn bind_uniform_block(
    uniform_block_name: &str,
    shader_program_name: &str,
    binding_point: u32,
    gfx: &GlData,
) {
    let ub_name = uniform_block_name.to_string() + "\0";
    let shd_id = gfx.get_shader_program_gl_id(shader_program_name);
    let uniform_block_idx = gl::GetUniformBlockIndex(
        shd_id,
        ub_name.as_ptr() as *const i8, /* Rustfmt force vertical formatting */
    );
    gl::UniformBlockBinding(
        shd_id,
        uniform_block_idx,
        binding_point, /* Rustfmt force vertical formatting */
    );
}

unsafe fn draw_floor(gfx: &GlData, shader_program_idx: usize, scale: f32) {
    let mut model_mat = Matrix4x4::new_scaling(scale, 1.0, scale);
    model_mat = Matrix4x4::new_translation(0.0, -1.0, 0.0) * model_mat;
    gfx.set_uniform_mat4x4("model_mat", shader_program_idx, &model_mat);
    gl::DrawArrays(gl::TRIANGLES, 12, 6);
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
    //gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    gl::BindVertexArray(gfx.vertex_array_objects[0]);
    gl::DrawArrays(gl::TRIANGLES, 0, 3);
}
