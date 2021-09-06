use super::{draw_floor, draw_two_containers};
use crate::gl;
use crate::gl::types::{GLenum, GLuint};
use crate::state_and_cfg::{GlData, State};
use mat_vec::{Matrix4x4, Vector3};
use opengl_learn::check_framebuffer_gl_status;
use std::fmt::{Display, Error, Formatter};

static mut REAR_VIEW_MIR_PP_OPTION: PostProcessingOption = PostProcessingOption::GaussianBlur5x5;

static mut DEFAULT_SHADER: usize = 0;
static mut POST_PROCESSING_SHADER: usize = 0;

static mut CONTAINER_TEXTURE: GLuint = 0;
static mut FLOOR_TEXTURE: GLuint = 0;

static mut MIRROR_FRAMEBUFFER: GLuint = 0;
static mut MIRROR_TEX_ATTACHMENT: GLuint = 0;
static mut REAR_VIEW_MIR_FRAMEBUFFER: GLuint = 0;
static mut REAR_VIEW_MIR_TEX_ATTACHMENT: GLuint = 0;
static mut POST_PROCESSING_FRAMEUFFER: GLuint = 0;
static mut POST_PROCESSING_TEX_ATTACHMENT: GLuint = 0;

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

pub unsafe fn draw_framebuffers(gfx: &GlData, projection_matrix: &Matrix4x4<f32>, state: &State) {
    gl::UseProgram(gfx.shader_programs[DEFAULT_SHADER]);

    let mirror_pos = Vector3::new(0.0, 2.0, -5.0);
    let mirror_norm = !Vector3::new(0.0, 0.0, 1.0);
    let (mirror_width, mirror_height) = (5.0, 5.0);

    gl::Enable(gl::DEPTH_TEST);

    // Draw to Mirror obj
    gl::BindFramebuffer(gl::FRAMEBUFFER, MIRROR_FRAMEBUFFER);
    gl::ClearColor(0.4, 0.5, 0.4, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    let (mirror_view_mat, mirror_proj_mat) = calculate_mirror_space_matrices(
        mirror_pos,
        mirror_norm,
        mirror_width,
        mirror_height,
        state.camera.position,
        state.camera.world_up_direction,
    );
    gfx.set_uniform_mat4x4("view_mat", DEFAULT_SHADER, &mirror_view_mat);
    gfx.set_uniform_mat4x4("projection_mat", DEFAULT_SHADER, &mirror_proj_mat);
    gl::BindTexture(gl::TEXTURE_2D, CONTAINER_TEXTURE);
    draw_two_containers(gfx, DEFAULT_SHADER, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, FLOOR_TEXTURE);
    draw_floor(gfx, DEFAULT_SHADER, 10.0);

    // Draw to rear-view mirror
    gl::BindFramebuffer(gl::FRAMEBUFFER, REAR_VIEW_MIR_FRAMEBUFFER);
    gl::ClearColor(0.4, 0.4, 0.4, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    let mut rear_view_mirror_camera = state.camera.clone();
    rear_view_mirror_camera.direction *= -1;
    rear_view_mirror_camera.recalculate_look_at_matrix();
    gfx.set_uniform_mat4x4(
        "view_mat",
        DEFAULT_SHADER,
        &rear_view_mirror_camera.look_at_matrix,
    );
    let mut flip_x_projection_mat = projection_matrix.clone();
    let e_r0c0 = flip_x_projection_mat.get(0, 0);
    flip_x_projection_mat.set(0, 0, -e_r0c0);
    gfx.set_uniform_mat4x4("projection_mat", DEFAULT_SHADER, &flip_x_projection_mat);
    gl::BindTexture(gl::TEXTURE_2D, CONTAINER_TEXTURE);
    draw_two_containers(gfx, DEFAULT_SHADER, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, FLOOR_TEXTURE);
    draw_floor(gfx, DEFAULT_SHADER, 10.0);
    // Draw Mirror obj (draw to rear-view mirror)
    let mut mirror_obj_model_mat = Matrix4x4::new_scaling(mirror_width, mirror_height, 0.0);
    let (mx, my, mz) = mirror_pos.get_components();
    mirror_obj_model_mat = Matrix4x4::new_translation(mx, my, mz) * mirror_obj_model_mat;
    gfx.set_uniform_mat4x4("model_mat", DEFAULT_SHADER, &mirror_obj_model_mat);
    gl::BindTexture(gl::TEXTURE_2D, MIRROR_TEX_ATTACHMENT);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    // Draw to texture
    gl::BindFramebuffer(gl::FRAMEBUFFER, POST_PROCESSING_FRAMEUFFER);
    gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    gfx.set_uniform_mat4x4("view_mat", DEFAULT_SHADER, &state.camera.look_at_matrix);
    gfx.set_uniform_mat4x4("projection_mat", DEFAULT_SHADER, projection_matrix);
    gl::BindTexture(gl::TEXTURE_2D, CONTAINER_TEXTURE);
    draw_two_containers(gfx, DEFAULT_SHADER, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, FLOOR_TEXTURE);
    draw_floor(gfx, DEFAULT_SHADER, 10.0);
    // Draw Mirror obj (draw to texture)
    gfx.set_uniform_mat4x4("model_mat", DEFAULT_SHADER, &mirror_obj_model_mat);
    gl::BindTexture(gl::TEXTURE_2D, MIRROR_TEX_ATTACHMENT);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);

    {
        gl::Disable(gl::DEPTH_TEST);
        gl::UseProgram(gfx.shader_programs[POST_PROCESSING_SHADER]);
        let identity_mat = Matrix4x4::identity_matrix();
        gfx.set_uniform_mat4x4("view_mat", POST_PROCESSING_SHADER, &identity_mat);
        gfx.set_uniform_mat4x4("projection_mat", POST_PROCESSING_SHADER, &identity_mat);

        // Draw to screen with post-processing
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        gl::ClearColor(1.0, 1.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
        let screen_quad_scaling = Matrix4x4::new_scaling(2.0, 2.0, 1.0);
        gfx.set_uniform_mat4x4("model_mat", POST_PROCESSING_SHADER, &screen_quad_scaling);
        gfx.set_uniform_1i(
            "mode",
            POST_PROCESSING_SHADER,
            state.post_processing_option.int_code(),
        );
        gl::BindTexture(gl::TEXTURE_2D, POST_PROCESSING_TEX_ATTACHMENT);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);

        // Draw rear-view mirror
        let mut model_mat = Matrix4x4::new_scaling(1.0, 0.5, 1.0);
        model_mat = Matrix4x4::new_translation(0.0, 0.7, 0.0) * model_mat;
        gfx.set_uniform_mat4x4("model_mat", POST_PROCESSING_SHADER, &model_mat);
        gfx.set_uniform_1i(
            "mode",
            POST_PROCESSING_SHADER,
            REAR_VIEW_MIR_PP_OPTION.int_code(),
        );
        gl::BindTexture(gl::TEXTURE_2D, REAR_VIEW_MIR_TEX_ATTACHMENT);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
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
    // imaginary viewer from opposite side (to real viewer) of the mirror
    let mirror_camera_pos = 2.0 * closest_point_to_cam - camera_pos;
    let mirror_view_matrix =
        Matrix4x4::new_LookAt_matrix(mirror_camera_pos, mirror_norm, world_up_dir);

    // Projection Matrix
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

pub unsafe fn setup_framebuffers(gfx: &mut GlData, window_size: (i32, i32)) {
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);
    DEFAULT_SHADER = gfx.get_shader_program_index("Blending shader");
    POST_PROCESSING_SHADER = gfx.get_shader_program_index("Post-processing shader");
    CONTAINER_TEXTURE = gfx.get_texture_gl_id("Container texture");
    FLOOR_TEXTURE = gfx.get_texture_gl_id("Metal texture");

    println!();
    let fb_name = "Framebuffer 1";
    let tex_att_name = "Texture Attachment 1";
    create_framebuffer(
        gfx,
        window_size,
        fb_name,
        tex_att_name,
        gl::NEAREST,
        gl::NEAREST,
    );
    POST_PROCESSING_FRAMEUFFER = gfx.get_framebuffer_gl_id(fb_name);
    POST_PROCESSING_TEX_ATTACHMENT = gfx.get_texture_attachment_gl_id(tex_att_name);

    let fb_name = "Mirror reflection framebuffer";
    let tex_att_name = "Mirror reflection texture attachment";
    create_framebuffer(
        gfx,
        window_size,
        fb_name,
        tex_att_name,
        gl::NEAREST,
        gl::NEAREST,
    );
    MIRROR_FRAMEBUFFER = gfx.get_framebuffer_gl_id(fb_name);
    MIRROR_TEX_ATTACHMENT = gfx.get_texture_attachment_gl_id(tex_att_name);

    let fb_name = "Mirror Rear-view framebuffer";
    let tex_att_name = "Mirror Rear-view texture attachment";
    create_framebuffer(
        gfx,
        window_size,
        fb_name,
        tex_att_name,
        gl::NEAREST,
        gl::NEAREST,
    );
    REAR_VIEW_MIR_FRAMEBUFFER = gfx.get_framebuffer_gl_id(fb_name);
    REAR_VIEW_MIR_TEX_ATTACHMENT = gfx.get_texture_attachment_gl_id(tex_att_name);
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

    pub fn from_int_code(code: i32) -> PostProcessingOption {
        use PostProcessingOption::*;
        match code {
            0 => None,
            1 => Inversion,
            2 => Grayscale,
            3 => GrayscalePhysicallyAccurate,
            4 => SharpenKernel,
            5 => GaussianBlur3x3,
            6 => EdgeDetection,
            7 => GaussianBlur5x5,
            8 => EmbossKernel,
            9 => BoxBlur,
            10 => SharpenKernel2,
            11 => CustomKernel,
            12 => CustomKernel2,
            13 => VerticalEdgeDetection,
            _ => None,
        }
    }
}

impl Display for PostProcessingOption {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use PostProcessingOption::*;
        let text = match self {
            None => "None",
            Inversion => "Inversion",
            Grayscale => "Grayscale",
            GrayscalePhysicallyAccurate => "Physically Accurate Grayscale",
            SharpenKernel => "Sharpen",
            GaussianBlur3x3 => "Gaussian Blur 3x3",
            EdgeDetection => "Edge Detection",
            GaussianBlur5x5 => "Gaussian Blur 5x5",
            EmbossKernel => "Emboss",
            BoxBlur => "Box Blur",
            SharpenKernel2 => "Sharpen 2",
            CustomKernel => "Custom Kernel",
            CustomKernel2 => "Custom Kernel 2",
            VerticalEdgeDetection => "Vertical Edge Detection",
        };
        write!(f, "{}", text)?;
        Ok(())
    }
}
