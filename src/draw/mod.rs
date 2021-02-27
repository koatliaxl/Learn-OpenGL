mod backpack;
mod blending;
mod cubemap;
mod cubes;
mod depth_texting;
mod face_culling;
mod framebuffers;
mod lighting;
mod stencil_testing;
mod ubo_use;

use self::cubes::draw_cubes;
use self::{
    backpack::*, blending::*, cubemap::*, depth_texting::*, face_culling::*, framebuffers::*,
    lighting::*, stencil_testing::*, ubo_use::*,
};
use crate::gl;
use crate::state_and_cfg::{GlData, State};
use ::opengl_learn::Model;
use glfw::Window;
use matrix::Matrix4x4;
use std::ffi::c_void;
use Draw::*;

static DRAW: Draw = _AdvDataUse;

#[allow(unused)]
enum Draw {
    Triangle,
    Cubes,
    LightingScene,
    Backpack,
    DepthTestingScene,
    StencilTestingScene,
    BlendingScene,
    FaceCulling,
    FrameBuffers,
    CubeMap,
    UniformBufferObjectsUse,

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
            view_mat.as_ptr() as *const c_void,
        );
        gl::BufferSubData(
            gl::UNIFORM_BUFFER,
            mat_size as isize,
            mat_size as isize,
            projection_mat.as_ptr() as *const c_void,
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
            DepthTestingScene => draw_depth_testing_scene(
                gfx,
                &view_mat,
                &projection_mat,
                1, /* Rustfmt force vertical formatting */
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
            _ => {}
        }
    }
}

pub fn init_draw(gfx: &mut GlData, model: &mut Model, window: &Window) {
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        match DRAW {
            Triangle | Cubes => gl::ClearColor(0.2, 0.2, 0.7, 1.0),
            LightingScene => init_lighting_scene(&gfx),
            Backpack => setup_backpack_draw(gfx, model),
            DepthTestingScene => setup_depth_testing_scene(),
            StencilTestingScene => setup_stencil_testing_scene(),
            BlendingScene => setup_blending_scene(),
            FaceCulling => setup_face_culling(),
            FrameBuffers => setup_framebuffers(gfx, window),
            CubeMap => setup_cubemap_scene(gfx, model),
            UniformBufferObjectsUse => setup_ubo_use(gfx),

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

unsafe fn adv_data_use(gfx: &GlData) {
    gl::BindBuffer(gl::ARRAY_BUFFER, gfx.array_buffers[1]);
    let size = 8 * 12;
    print_gl_buffer(gl::ARRAY_BUFFER, size, 8);
    #[allow(unused_imports)]
    use gl::{COPY_READ_BUFFER, COPY_WRITE_BUFFER};
    gl::BindBuffer(COPY_READ_BUFFER, gfx.array_buffers[4]);
    print_gl_buffer(COPY_READ_BUFFER, 3 * 8, 3);
    gl::CopyBufferSubData(gl::ARRAY_BUFFER, COPY_READ_BUFFER, 0, 0, 2 * 4);
    print_gl_buffer(COPY_READ_BUFFER, 3 * 8, 3);

    let ptr = gl::MapBuffer(COPY_READ_BUFFER, gl::WRITE_ONLY);
    let mut ptr = ptr as *mut f32;
    *ptr = 9.0;
    ptr = ptr.offset(2);
    *ptr = 9.9;
    if gl::UnmapBuffer(COPY_READ_BUFFER) == gl::TRUE {
        println!("\nGL data was mapped successfully");
    } else {
        println!("\nGL data mapping was unsuccessful");
    }
    print_gl_buffer(COPY_READ_BUFFER, 3 * 8, 3);
}

unsafe fn print_gl_buffer(target: gl::types::GLenum, size: usize, format: usize) {
    #[allow(unused_imports)]
    use gl::{READ_ONLY, READ_WRITE, WRITE_ONLY};
    let ptr = gl::MapBuffer(target, READ_ONLY);
    let mut arr = [0_f32; 200];
    (ptr as *const f32).copy_to(arr.as_mut_ptr(), size);
    let ret = gl::UnmapBuffer(target);
    if ret == gl::TRUE {
        println!("\nGL data was mapped successfully");
    } else {
        println!("\nGL data mapping was unsuccessful");
    }
    for i in 0..size {
        print!("{}, ", arr[i]);
        if format != 0 && (i + 1) % format == 0 {
            println!();
        }
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
