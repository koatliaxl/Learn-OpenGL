use crate::gl;
use crate::gl::types::GLsizei;
use crate::gl::{FRAMEBUFFER, TEXTURE_2D};
use crate::state_and_cfg::GlData;
use glfw::Window;

static SHADOW_WIDTH: GLsizei = 1024;
static SHADOW_HEIGHT: GLsizei = 1024;

pub unsafe fn draw_shadow_mapping(gfx: &GlData, window: &Window) {
    // Render to the depth map
    gl::Viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
    let fbo_id = gfx.get_framebuffer_gl_id("Depth/Shadow Map");
    gl::BindFramebuffer(FRAMEBUFFER, fbo_id);
    gl::Clear(gl::DEPTH_BUFFER_BIT);
    //todo: configure shader and matrices
    //todo: render scene

    // Render scene as normal
    gl::BindFramebuffer(FRAMEBUFFER, 0);
    gl::Viewport(0, 0, window.get_size().0, window.get_size().1);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    //todo: configure shader and matrices
    let tex_id = gfx.get_texture_attachment_gl_id("Depth Map");
    gl::BindTexture(TEXTURE_2D, tex_id);
    //todo: render scene
}

pub unsafe fn setup_shadow_mapping(gfx: &mut GlData) {
    let mut depth_map = 0;
    gl::GenFramebuffers(1, &mut depth_map);
    gfx.add_framebuffer(depth_map, "Depth/Shadow Map");
    gl::BindFramebuffer(FRAMEBUFFER, depth_map);

    let mut depth_map_tex = 0;
    gl::GenTextures(1, &mut depth_map_tex);
    gfx.add_texture_attachment(depth_map_tex, "Depth Map");
    gl::BindTexture(TEXTURE_2D, depth_map_tex);
    gl::TexImage2D(
        TEXTURE_2D,
        0,
        gl::DEPTH_COMPONENT as i32,
        SHADOW_WIDTH,
        SHADOW_HEIGHT,
        0,
        gl::DEPTH_COMPONENT,
        gl::FLOAT,
        std::ptr::null(),
    );
    gl::TexParameteri(TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

    gl::FramebufferTexture2D(
        FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        TEXTURE_2D,
        depth_map_tex,
        0,
    );
    gl::DrawBuffer(gl::NONE);
    gl::ReadBuffer(gl::NONE);
}
