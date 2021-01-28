use super::{draw_floor, draw_two_containers};
use crate::camera::Camera;
use crate::gl;
use crate::gl::types::GLenum;
use crate::state_and_cfg::GlData;
use glfw::Window;
use matrix::{Matrix4x4, Vector3};

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum PostProcessingOption {
    None,
    Inversion,
    Grayscale,
    GrayscalePhysicallyAccurate,
    SharpenKernel,
    GaussianBlur3x3,
    EdgeDetection,
    GaussianBlur5x5,
    EmbossKernel,
    BoxBlur,
}

pub unsafe fn draw_framebuffers(
    gfx: &GlData,
    //view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    mode: PostProcessingOption,
    camera: Camera,
) {
    gl::UseProgram(gfx.shader_programs[7]);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);

    gl::Enable(gl::DEPTH_TEST);

    // Draw to Mirror obj
    gl::BindFramebuffer(gl::FRAMEBUFFER, gfx.framebuffers[2]);
    gl::ClearColor(0.4, 0.5, 0.4, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    let mirror_pos = Vector3::new(0.0, 2.0, -5.0);
    let mirror_norm = !Vector3::new(0.0, 0.0, 1.0);
    let cam_to_mir_dist = (camera.position - mirror_pos) % mirror_norm;
    // closest point to cameras (real and mirror/"imaginary") on mirror plane
    let closest_point_to_cam = camera.position - mirror_norm * cam_to_mir_dist;
    let mut mirror_camera = camera.clone();
    mirror_camera.position = 2.0 * closest_point_to_cam - camera.position;
    mirror_camera.direction = mirror_norm;
    mirror_camera.recalculate_look_at_matrix();
    gfx.set_uniform_mat4x4("view_mat", 7, &mirror_camera.look_at_matrix);
    let mirror_right_axis = !Vector3::new(-1.0, 0.0, 0.0);
    let mirror_top_axis = !Vector3::new(0.0, 1.0, 0.0);
    // mirror position to cam projection point (on mirror plane) vector
    let mir_to_point_vec = closest_point_to_cam - mirror_pos;
    let proj_on_r_axis = mir_to_point_vec % mirror_right_axis;
    let proj_on_t_axis = mir_to_point_vec % mirror_top_axis;
    let (mirror_width, mirror_height) = (5.0, 5.0);
    let proj_plane_right = -mirror_width / 2.0 - proj_on_r_axis;
    let proj_plane_left = mirror_width / 2.0 - proj_on_r_axis;
    let proj_plane_top = mirror_height / 2.0 - proj_on_t_axis;
    let proj_plane_bottom = -mirror_height / 2.0 - proj_on_t_axis;
    let mirror_projection_mat = Matrix4x4::new_perspective_projection_2(
        proj_plane_right,
        proj_plane_left,
        proj_plane_top,
        proj_plane_bottom,
        100.0,
        cam_to_mir_dist,
    );
    gfx.set_uniform_mat4x4("projection_mat", 7, &mirror_projection_mat);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[2]);
    draw_two_containers(gfx, 7, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 7);

    gfx.set_uniform_mat4x4("projection_mat", 7, projection_matrix);

    // Draw to rear-view mirror
    gl::BindFramebuffer(gl::FRAMEBUFFER, gfx.framebuffers[1]);
    gl::ClearColor(0.4, 0.4, 0.4, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    let mut rear_view_mirror_camera = camera.clone();
    rear_view_mirror_camera.direction = -camera.direction;
    rear_view_mirror_camera.recalculate_look_at_matrix();
    gfx.set_uniform_mat4x4("view_mat", 7, &rear_view_mirror_camera.look_at_matrix);
    let mut flip_x_projection_mat = projection_matrix.clone();
    let e_r0c0 = flip_x_projection_mat.get(0, 0);
    flip_x_projection_mat.set(0, 0, -e_r0c0);
    gfx.set_uniform_mat4x4("projection_mat", 7, &flip_x_projection_mat);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[2]);
    draw_two_containers(gfx, 7, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 7);
    // Draw Mirror obj (draw to rear-view mirror)
    let mut mirror_obj_model_mat = Matrix4x4::new_scaling(mirror_width, mirror_height, 0.0);
    let (mx, my, mz) = mirror_pos.get_components();
    mirror_obj_model_mat = Matrix4x4::new_translation(mx, my, mz) * mirror_obj_model_mat;
    gfx.set_uniform_mat4x4("model_mat", 7, &mirror_obj_model_mat);
    gl::BindTexture(gl::TEXTURE_2D, gfx.texture_attachments[2]);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    gfx.set_uniform_mat4x4("projection_mat", 7, projection_matrix);

    // Draw to texture
    gl::BindFramebuffer(gl::FRAMEBUFFER, gfx.framebuffers[0]);
    gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    gfx.set_uniform_mat4x4("view_mat", 7, &camera.look_at_matrix);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[2]);
    draw_two_containers(gfx, 7, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 7);
    // Draw Mirror obj (draw to texture)
    gfx.set_uniform_mat4x4("model_mat", 7, &mirror_obj_model_mat);
    gl::BindTexture(gl::TEXTURE_2D, gfx.texture_attachments[2]);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    gl::UseProgram(gfx.shader_programs[8]);
    let identity_mat = Matrix4x4::identity_matrix();
    gfx.set_uniform_mat4x4("view_mat", 8, &identity_mat);
    gfx.set_uniform_mat4x4("projection_mat", 8, &identity_mat);
    gl::Disable(gl::DEPTH_TEST);

    // Draw to screen with post-processing
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    let model_mat = Matrix4x4::new_scaling(2.0, 2.0, 0.0);
    gfx.set_uniform_mat4x4("model_mat", 8, &model_mat);
    gfx.set_uniform_1i("mode", 8, mode.int_code());
    gl::BindTexture(gl::TEXTURE_2D, gfx.texture_attachments[0]);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    // Draw rear-view mirror
    let mut model_mat = Matrix4x4::new_scaling(1.0, 0.5, 0.0);
    model_mat = Matrix4x4::new_translation(0.0, 0.7, 0.0) * model_mat;
    gfx.set_uniform_mat4x4("model_mat", 8, &model_mat);
    gfx.set_uniform_1i("mode", 8, PostProcessingOption::GaussianBlur5x5.int_code());
    gl::BindTexture(gl::TEXTURE_2D, gfx.texture_attachments[1]);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
}

pub unsafe fn setup_framebuffers(gfx: &mut GlData, window: &Window) {
    println!();
    create_framebuffer(
        gfx,
        window,
        "Framebuffer 1",
        gl::NEAREST,
        gl::NEAREST, /* Rustfmt force vertical formatting */
    );
    create_framebuffer(
        gfx,
        window,
        "Mirror Rear-view framebuffer",
        gl::LINEAR,
        gl::LINEAR,
    );
    create_framebuffer(
        gfx,
        window,
        "Mirror reflection framebuffer",
        gl::LINEAR,
        gl::LINEAR,
    );
}

#[allow(unused_imports)]
unsafe fn create_framebuffer(
    gfx: &mut GlData,
    window: &Window,
    name: &str,
    tex_min_filter: GLenum,
    tex_mag_filter: GLenum,
) {
    let mut fbo_id = 0;
    gl::GenFramebuffers(1, &mut fbo_id);
    use gl::{DRAW_FRAMEBUFFER, FRAMEBUFFER, READ_FRAMEBUFFER};
    gl::BindFramebuffer(FRAMEBUFFER, fbo_id);

    let mut tex_color_buf = 0;
    gl::GenTextures(1, &mut tex_color_buf);
    gl::BindTexture(gl::TEXTURE_2D, tex_color_buf);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        window.get_size().0,
        window.get_size().1,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        std::ptr::null(),
    );
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MIN_FILTER,
        tex_min_filter as i32,
    );
    gl::TexParameteri(
        gl::TEXTURE_2D,
        gl::TEXTURE_MAG_FILTER,
        tex_mag_filter as i32,
    );
    use gl::{
        COLOR_ATTACHMENT0, /*... */
        DEPTH_ATTACHMENT, DEPTH_STENCIL_ATTACHMENT, STENCIL_ATTACHMENT,
    };
    gl::FramebufferTexture2D(
        FRAMEBUFFER,
        COLOR_ATTACHMENT0,
        gl::TEXTURE_2D,
        tex_color_buf,
        0,
    );

    /*gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::DEPTH24_STENCIL8 as i32,
        window.get_size().0,
        window.get_size().1,
        0,
        gl::DEPTH_STENCIL,
        gl::UNSIGNED_INT_24_8,
        std::ptr::null(),
    );*/

    let mut render_buffer = 0;
    gl::GenRenderbuffers(1, &mut render_buffer);
    gl::BindRenderbuffer(gl::RENDERBUFFER, render_buffer);
    gl::RenderbufferStorage(
        gl::RENDERBUFFER,
        gl::DEPTH24_STENCIL8,
        window.get_size().0,
        window.get_size().1,
    );
    gl::FramebufferRenderbuffer(
        FRAMEBUFFER,
        DEPTH_STENCIL_ATTACHMENT,
        gl::RENDERBUFFER,
        render_buffer,
    );

    let status = match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
        gl::FRAMEBUFFER_COMPLETE => "COMPLETE",
        gl::FRAMEBUFFER_UNDEFINED => "UNDEFINED",
        gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => "INCOMPLETE_ATTACHMENT",
        gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => "INCOMPLETE_MISSING_ATTACHMENT",
        gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => "INCOMPLETE_DRAW_BUFFER",
        gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => "INCOMPLETE_READ_BUFFER",
        gl::FRAMEBUFFER_UNSUPPORTED => "UNSUPPORTED",
        gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => "INCOMPLETE_MULTISAMPLE",
        gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => "INCOMPLETE_LAYER_TARGETS",
        _ => "Error",
    };
    println!("{1:} status: {}", status, name);

    gfx.framebuffers.push(fbo_id);
    gfx.texture_attachments.push(tex_color_buf);
    gfx.render_buffer_attachments.push(render_buffer);
}

impl PostProcessingOption {
    fn int_code(&self) -> i32 {
        match self {
            PostProcessingOption::None => 0,
            PostProcessingOption::Inversion => 1,
            PostProcessingOption::Grayscale => 2,
            PostProcessingOption::GrayscalePhysicallyAccurate => 3,
            PostProcessingOption::SharpenKernel => 4,
            PostProcessingOption::GaussianBlur3x3 => 5,
            PostProcessingOption::EdgeDetection => 6,
            PostProcessingOption::GaussianBlur5x5 => 7,
            PostProcessingOption::EmbossKernel => 8,
            PostProcessingOption::BoxBlur => 9,
        }
    }
}
