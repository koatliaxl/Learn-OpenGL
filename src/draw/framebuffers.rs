use super::{draw_floor, draw_two_containers};
use crate::gl;
use crate::state_and_cfg::GlData;
use glfw::Window;
use matrix::Matrix4x4;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum PostProcessingOption {
    None,
    Inversion,
    Grayscale,
    GrayscalePhysicallyAccurate,
    SharpenKernel,
    Blur,
    EdgeDetection,
}

pub unsafe fn draw_framebuffers(
    gfx: &GlData,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    mode: PostProcessingOption,
) {
    gl::UseProgram(gfx.shader_programs[7]);
    gfx.set_uniform_mat4x4("view_mat", 7, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 7, projection_matrix);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE0);

    gl::BindFramebuffer(gl::FRAMEBUFFER, gfx.framebuffers[0]);
    gl::ClearColor(0.2, 0.2, 0.2, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    gl::Enable(gl::DEPTH_TEST);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[2]);
    draw_two_containers(gfx, 7, 1.0);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[6]);
    draw_floor(gfx, 7);

    gl::UseProgram(gfx.shader_programs[8]);
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    gl::ClearColor(1.0, 1.0, 1.0, 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
    gl::Disable(gl::DEPTH_TEST);
    /*let mut model_mat = Matrix4x4::new_x_rotation(10.0);
    model_mat = Matrix4x4::new_translation(0.0, 0.0, 2.0) * model_mat;
    gfx.set_uniform_mat4x4("model_mat", 8, &model_mat);*/
    let model_mat = Matrix4x4::new_scaling(2.0, 2.0, 2.0);
    let identity_mat = Matrix4x4::identity_matrix();
    gfx.set_uniform_mat4x4("model_mat", 8, &model_mat);
    gfx.set_uniform_mat4x4("view_mat", 8, &identity_mat);
    gfx.set_uniform_mat4x4("projection_mat", 8, &identity_mat);
    gfx.set_uniform_1i("mode", 8, mode.int_code());
    gl::BindTexture(gl::TEXTURE_2D, gfx.texture_attachments[0]);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
}

#[allow(unused_imports)]
pub unsafe fn setup_framebuffers(gfx: &mut GlData, window: &Window) {
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
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
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
    //gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
    gl::FramebufferRenderbuffer(
        FRAMEBUFFER,
        DEPTH_STENCIL_ATTACHMENT,
        gl::RENDERBUFFER,
        render_buffer,
    );

    let status = match gl::CheckFramebufferStatus(FRAMEBUFFER) {
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
    println!("\nFramebuffer status: {}", status);
    if let "COMPLETE" = status {
        gfx.framebuffers.push(fbo_id);
    }
    gfx.texture_attachments.push(tex_color_buf);
    gfx.render_buffer_attachments.push(render_buffer);
    //gl::BindFramebuffer(FRAMEBUFFER, 0);
}

impl PostProcessingOption {
    fn int_code(&self) -> i32 {
        match self {
            PostProcessingOption::None => 0,
            PostProcessingOption::Inversion => 1,
            PostProcessingOption::Grayscale => 2,
            PostProcessingOption::GrayscalePhysicallyAccurate => 3,
            PostProcessingOption::SharpenKernel => 4,
            PostProcessingOption::Blur => 5,
            PostProcessingOption::EdgeDetection => 6,
        }
    }
}
