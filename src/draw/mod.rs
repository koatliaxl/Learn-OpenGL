mod adv_data_use;
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

use self::cubes::draw_cubes;
use self::{
    adv_data_use::*, backpack::*, blending::*, cubemap::*, depth_texting::*, face_culling::*,
    framebuffers::*, geometry_shader_use::*, instancing::*, lighting::*, stencil_testing::*,
    ubo_use::*,
};
use crate::gl;
use crate::state_and_cfg::{GlData, State};
use ::opengl_learn::Model;
use glfw::Window;
use mat_vec::Matrix4x4;
use std::ffi::c_void;
use Draw::*;

static DRAW: Draw = Instancing(Asteroids);

#[allow(unused)]
enum Draw {
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

    _AdvDataUse,
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
                PostProcessingOption::CustomKernel2,
                state.camera.clone(),
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
            FrameBuffers => setup_framebuffers(gfx, window),
            CubeMap => setup_cubemap_scene(gfx, model),
            UniformBufferObjectsUse => setup_ubo_use(gfx),
            GeometryShaderUse(opt) => setup_geometry_shd_use(gfx, model, opt),
            Instancing(opt) => setup_instancing(gfx, opt, state),

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
        //gl::BindBufferBase(gl::UNIFORM_BUFFER, 0, ubo_matrices);
        gfx.insert_uniform_buffer(ubo_matrices, "Matrices");
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
    //gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
    gl::BindVertexArray(gfx.vertex_array_objects[0]);
    gl::DrawArrays(gl::TRIANGLES, 0, 3);
}
