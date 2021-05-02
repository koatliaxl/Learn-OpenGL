use super::framebuffers::{self, PostProcessingOption};
use crate::gl;
use crate::state_and_cfg::GlData;
use gl::TEXTURE_2D_MULTISAMPLE;
use mat_vec::Matrix4x4;
use AntiAliasingMode::*;

static mut WIDTH: i32 = 0;
static mut HEIGHT: i32 = 0;
static mut MODE: AntiAliasingMode = DirectOutput;

#[derive(PartialEq)]
pub enum AntiAliasingMode {
    DirectOutput,
    WithPostProcessing,
    CustomAntiAliasing,
}

pub unsafe fn draw_antialiasing(gfx: &GlData) {
    let ms_fb = gfx.get_framebuffer_gl_id("Multisampling framebuffer");
    gl::BindFramebuffer(gl::FRAMEBUFFER, ms_fb);
    //gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, ms_fb);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    gfx.use_shader_program("UBO Use shader 2");
    gl::DrawArrays(gl::TRIANGLES, 0, 36);

    //gl::BindFramebuffer(gl::READ_FRAMEBUFFER, ms_fb);
    if let DirectOutput = MODE {
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
        //gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
    if let WithPostProcessing = MODE {
        let im_fb = gfx.get_framebuffer_gl_id("Intermediate framebuffer");
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, im_fb);
    }
    if MODE == DirectOutput || MODE == WithPostProcessing {
        gl::BlitFramebuffer(
            0,
            0,
            WIDTH,
            HEIGHT,
            0,
            0,
            WIDTH,
            HEIGHT,
            gl::COLOR_BUFFER_BIT, /* | gl::DEPTH_BUFFER_BIT*/
            gl::NEAREST,
        );
    }

    if let WithPostProcessing = MODE {
        let im_tex = gfx.get_texture_attachment_gl_id("Intermediate texture attachment");
        gl::BindTexture(gl::TEXTURE_2D, im_tex);
        gfx.use_shader_program("Post-processing shader");
    }
    if let CustomAntiAliasing = MODE {
        //gl::Disable(gl::DEPTH_TEST);
        let ms_tex = gfx.get_texture_attachment_gl_id("Multisampling texture attachment");
        gl::BindTexture(gl::TEXTURE_2D_MULTISAMPLE, ms_tex);
        gfx.use_shader_program("Custom Anti-aliasing shader");
        //gl::Enable(gl::DEPTH_TEST);
    }
    if MODE == WithPostProcessing || MODE == CustomAntiAliasing {
        gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
}

pub unsafe fn setup_antialiasing(
    gfx: &mut GlData,
    window_size: (i32, i32),
    samples: u32,
    mode: AntiAliasingMode,
) {
    gl::Enable(gl::MULTISAMPLE);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    WIDTH = window_size.0;
    HEIGHT = window_size.1;
    MODE = mode;
    let identity_mat = Matrix4x4::identity_matrix();

    let shd_idx = gfx.get_shader_program_index("UBO Use shader 2");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &identity_mat);

    if let WithPostProcessing = MODE {
        let shd_idx = gfx.get_shader_program_index("Post-processing shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        let model_mat = Matrix4x4::new_scaling(2.0, 2.0, 2.0);
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        gfx.set_uniform_mat4x4("view_mat", shd_idx, &identity_mat);
        gfx.set_uniform_mat4x4("projection_mat", shd_idx, &identity_mat);
        gfx.set_uniform_1i(
            "mode",
            shd_idx,
            PostProcessingOption::Grayscale.int_code(), /* Rustfmt force vertical formatting */
        );

        framebuffers::create_framebuffer(
            gfx,
            window_size,
            "Intermediate framebuffer",
            "Intermediate texture attachment",
            gl::NEAREST,
            gl::NEAREST,
        );
    }

    if let CustomAntiAliasing = MODE {
        let shd_idx = gfx.get_shader_program_index("Custom Anti-aliasing shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        let model_mat = Matrix4x4::new_scaling(2.0, 2.0, 2.0);
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        gfx.set_uniform_mat4x4("view_mat", shd_idx, &identity_mat);
        gfx.set_uniform_mat4x4("projection_mat", shd_idx, &identity_mat);
    }

    let mut multi_sample_buf = 0;
    gl::GenFramebuffers(1, &mut multi_sample_buf);
    gl::BindFramebuffer(gl::FRAMEBUFFER, multi_sample_buf);
    gfx.add_framebuffer(multi_sample_buf, "Multisampling framebuffer");

    let mut ms_tex = 0;
    gl::GenTextures(1, &mut ms_tex);
    gl::BindTexture(TEXTURE_2D_MULTISAMPLE, ms_tex);
    gl::TexImage2DMultisample(
        TEXTURE_2D_MULTISAMPLE,
        samples as i32,
        gl::RGB,
        window_size.0,
        window_size.1,
        gl::TRUE,
    );
    gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::COLOR_ATTACHMENT0,
        TEXTURE_2D_MULTISAMPLE,
        ms_tex,
        0,
    );
    gfx.add_texture_attachment(ms_tex, "Multisampling texture attachment");

    let mut ms_renderbuffer = 0;
    gl::GenRenderbuffers(1, &mut ms_renderbuffer);
    gl::BindRenderbuffer(gl::RENDERBUFFER, ms_renderbuffer);
    gl::RenderbufferStorageMultisample(
        gl::RENDERBUFFER,
        samples as i32,
        gl::DEPTH24_STENCIL8, /*gl::RGB*/
        window_size.0,
        window_size.1,
    );
    gl::FramebufferRenderbuffer(
        gl::FRAMEBUFFER,
        gl::DEPTH_STENCIL_ATTACHMENT, /*gl::COLOR_ATTACHMENT0*/
        gl::RENDERBUFFER,
        ms_renderbuffer,
    );
    gfx.render_buffer_attachments.push(ms_renderbuffer);
}
