use super::{draw_floor, draw_two_containers};
use crate::camera::Camera;
use crate::gl;
use crate::gl::types::GLenum;
use crate::state_and_cfg::GlData;
use mat_vec::{Matrix4x4, Vector3};
use opengl_learn::check_framebuffer_gl_status;

static mut POST_PROCESSING_OPTION: PostProcessingOption = PostProcessingOption::None;
static mut REAR_VIEW_MIRROR_POST_PROCESSING_OPTION: PostProcessingOption =
    PostProcessingOption::None;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum PostProcessingOption {
    None,
    Inversion,
    Grayscale,
    GrayscalePhysicallyAccurate,
    SharpenKernel,
    SharpenKernel2,
    GaussianBlur3x3,
    EdgeDetection,
    GaussianBlur5x5,
    EmbossKernel,
    BoxBlur,
    CustomKernel,
    CustomKernel2,
    VerticalEdgeDetection,
}

pub unsafe fn draw_framebuffers(
    gfx: &GlData,
    //view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    camera: Camera,
) {
    gl::UseProgram(gfx.shader_programs[7]);

    let mirror_pos = Vector3::new(0.0, 2.0, -5.0);
    let mirror_norm = !Vector3::new(0.0, 0.0, 1.0);
    let (mirror_width, mirror_height) = (5.0, 5.0);

    gl::Enable(gl::DEPTH_TEST);

    // Draw to Mirror obj
    let fb_id = gfx.get_framebuffer_gl_id("Mirror reflection framebuffer");
    gl::BindFramebuffer(gl::FRAMEBUFFER, fb_id);
    gl::ClearColor(0.4, 0.5, 0.4, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    let (mirror_view_mat, mirror_proj_mat) = calculate_mirror_space_matrices(
        mirror_pos,
        mirror_norm,
        mirror_width,
        mirror_height,
        camera.position,
        camera.world_up_direction,
    );
    gfx.set_uniform_mat4x4("view_mat", 7, &mirror_view_mat);
    gfx.set_uniform_mat4x4("projection_mat", 7, &mirror_proj_mat);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[2]);
    draw_two_containers(gfx, 7, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 7, 10.0);

    gfx.set_uniform_mat4x4("projection_mat", 7, projection_matrix);

    // Draw to rear-view mirror
    let fb_id = gfx.get_framebuffer_gl_id("Mirror Rear-view framebuffer");
    gl::BindFramebuffer(gl::FRAMEBUFFER, fb_id);
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
    draw_floor(gfx, 7, 10.0);
    // Draw Mirror obj (draw to rear-view mirror)
    let mut mirror_obj_model_mat = Matrix4x4::new_scaling(mirror_width, mirror_height, 0.0);
    let (mx, my, mz) = mirror_pos.get_components();
    mirror_obj_model_mat = Matrix4x4::new_translation(mx, my, mz) * mirror_obj_model_mat;
    gfx.set_uniform_mat4x4("model_mat", 7, &mirror_obj_model_mat);
    let tex_att_id = gfx.get_texture_attachment_gl_id("Mirror reflection texture attachment");
    gl::BindTexture(gl::TEXTURE_2D, tex_att_id);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    gfx.set_uniform_mat4x4("projection_mat", 7, projection_matrix);

    // Draw to texture
    let fb_id = gfx.get_framebuffer_gl_id("Framebuffer 1");
    gl::BindFramebuffer(gl::FRAMEBUFFER, fb_id);
    gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    gfx.set_uniform_mat4x4("view_mat", 7, &camera.look_at_matrix);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[2]);
    draw_two_containers(gfx, 7, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 7, 10.0);
    // Draw Mirror obj (draw to texture)
    gfx.set_uniform_mat4x4("model_mat", 7, &mirror_obj_model_mat);
    gl::BindTexture(gl::TEXTURE_2D, tex_att_id);
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
    gfx.set_uniform_1i("mode", 8, POST_PROCESSING_OPTION.int_code());
    let tex_att_id = gfx.get_texture_attachment_gl_id("Texture Attachment 1");
    gl::BindTexture(gl::TEXTURE_2D, tex_att_id);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    // Draw rear-view mirror
    let mut model_mat = Matrix4x4::new_scaling(1.0, 0.5, 0.0);
    model_mat = Matrix4x4::new_translation(0.0, 0.7, 0.0) * model_mat;
    gfx.set_uniform_mat4x4("model_mat", 8, &model_mat);
    gfx.set_uniform_1i(
        "mode",
        8,
        REAR_VIEW_MIRROR_POST_PROCESSING_OPTION.int_code(), /* Rustfmt force vertical formatting */
    );
    let tex_att_id = gfx.get_texture_attachment_gl_id("Mirror Rear-view texture attachment");
    gl::BindTexture(gl::TEXTURE_2D, tex_att_id);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
}

unsafe fn calculate_mirror_space_matrices(
    mirror_pos: Vector3<f32>,
    mirror_norm: Vector3<f32>,
    mirror_width: f32,
    mirror_height: f32,
    camera_pos: Vector3<f32>,
    world_up_dir: Vector3<f32>,
) -> (Matrix4x4<f32>, Matrix4x4<f32>) {
    // View matrix
    let cam_to_mir_dist = (camera_pos - mirror_pos) % mirror_norm;
    // closest point to cameras (real and mirror/"imaginary") on mirror plane
    let closest_point_to_cam = camera_pos - mirror_norm * cam_to_mir_dist;
    //let mut mirror_camera = camera.clone();
    // imaginary viewer from opposite (to real viewer) side of the mirror
    let mirror_camera_pos = 2.0 * closest_point_to_cam - camera_pos;
    /*mirror_camera.direction = mirror_norm;
    mirror_camera.recalculate_look_at_matrix();*/
    let mirror_view_matrix =
        Matrix4x4::new_LookAt_matrix(mirror_camera_pos, mirror_norm, world_up_dir);

    // Projection Matrix
    /*let mirror_right_axis = !Vector3::new(-1.0, 0.0, 0.0);
    let mirror_top_axis = !Vector3::new(0.0, 1.0, 0.0);*/
    let mirror_right_axis = !(mirror_norm ^ world_up_dir);
    let mirror_top_axis = !(mirror_right_axis ^ mirror_norm);
    // mirror position to cam projection point (on mirror plane) vector
    let mir_to_point_vec = closest_point_to_cam - mirror_pos;
    let proj_on_r_axis = mir_to_point_vec % mirror_right_axis;
    let proj_on_t_axis = mir_to_point_vec % mirror_top_axis;
    let proj_plane_right = -mirror_width / 2.0 - proj_on_r_axis;
    let proj_plane_left = mirror_width / 2.0 - proj_on_r_axis;
    let proj_plane_top = mirror_height / 2.0 - proj_on_t_axis;
    let proj_plane_bottom = -mirror_height / 2.0 - proj_on_t_axis;
    let mirror_projection_mat = Matrix4x4::new_perspective_projection_by_dimensions(
        proj_plane_right,
        proj_plane_left,
        proj_plane_top,
        proj_plane_bottom,
        100.0,
        cam_to_mir_dist,
    );

    (mirror_view_matrix, mirror_projection_mat)
}

pub unsafe fn setup_framebuffers(
    gfx: &mut GlData,
    window_size: (i32, i32),
    post_processing_opt: PostProcessingOption,
    rear_view_mirror_post_processing_opt: PostProcessingOption,
) {
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);
    POST_PROCESSING_OPTION = post_processing_opt;
    REAR_VIEW_MIRROR_POST_PROCESSING_OPTION = rear_view_mirror_post_processing_opt;
    println!();
    create_framebuffer(
        gfx,
        window_size,
        "Framebuffer 1",
        "Texture Attachment 1",
        gl::NEAREST,
        gl::NEAREST, /* Rustfmt force vertical formatting */
    );
    create_framebuffer(
        gfx,
        window_size,
        "Mirror Rear-view framebuffer",
        "Mirror Rear-view texture attachment",
        gl::LINEAR,
        gl::LINEAR,
    );
    create_framebuffer(
        gfx,
        window_size,
        "Mirror reflection framebuffer",
        "Mirror reflection texture attachment",
        gl::LINEAR,
        gl::LINEAR,
    );
}

#[allow(unused_imports)]
pub unsafe fn create_framebuffer(
    gfx: &mut GlData,
    window_size: (i32, i32),
    name: &str,
    tex_attachment_name: &str,
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
        window_size.0,
        window_size.1,
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
        window_size.0,
        window_size.1,
    );
    gl::FramebufferRenderbuffer(
        FRAMEBUFFER,
        DEPTH_STENCIL_ATTACHMENT,
        gl::RENDERBUFFER,
        render_buffer,
    );

    let status = check_framebuffer_gl_status();
    println!("\"{1:}\" status: {}", status, name);

    gfx.add_framebuffer(fbo_id, name);
    gfx.add_texture_attachment(tex_color_buf, tex_attachment_name);
    gfx.render_buffer_attachments.push(render_buffer);
}

impl PostProcessingOption {
    pub fn int_code(&self) -> i32 {
        use PostProcessingOption::*;
        match self {
            None => 0,
            Inversion => 1,
            Grayscale => 2,
            GrayscalePhysicallyAccurate => 3,
            SharpenKernel => 4,
            GaussianBlur3x3 => 5,
            EdgeDetection => 6,
            GaussianBlur5x5 => 7,
            EmbossKernel => 8,
            BoxBlur => 9,
            SharpenKernel2 => 10,
            CustomKernel => 11,
            CustomKernel2 => 12,
            VerticalEdgeDetection => 13,
        }
    }
}
