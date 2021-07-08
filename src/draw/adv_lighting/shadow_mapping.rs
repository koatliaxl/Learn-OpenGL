use crate::draw::draw_floor;
use crate::gl;
use crate::gl::types::{GLsizei, GLuint};
use crate::gl::{FRAMEBUFFER, TEXTURE_2D};
use crate::state_and_cfg::GlData;
use glfw::Window;
use mat_vec::{Matrix4x4, Vector3};

static SHADOW_WIDTH: GLsizei = 1024;
static SHADOW_HEIGHT: GLsizei = 1024;
static mut CUBES_MODEL_MATRICES: Vec<Matrix4x4<f32>> = Vec::new();

static mut TO_SCREEN_SHADER: GLuint = 0;

pub unsafe fn draw_shadow_mapping(gfx: &GlData, window: &Window) {
    // Render to the depth map
    gl::Viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
    let fbo_id = gfx.get_framebuffer_gl_id("Depth/Shadow Map");
    gl::BindFramebuffer(FRAMEBUFFER, fbo_id);
    gl::Clear(gl::DEPTH_BUFFER_BIT);
    let shd_idx = gfx.get_shader_program_index("Depth/Shadow Map shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);

    let tex_id = gfx.get_texture_gl_id("Wood Flooring");
    gl::BindTexture(TEXTURE_2D, tex_id);

    /*let fbo_id = gfx.get_framebuffer_gl_id("Temp framebuffer");
    gl::BindFramebuffer(FRAMEBUFFER, fbo_id);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    let shd_idx = gfx.get_shader_program_index("UB Default shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let tex_id = gfx.get_texture_gl_id("Wood Flooring");
    gl::BindTexture(TEXTURE_2D, tex_id);*/

    let mut floor_model_mat = Matrix4x4::new_scaling(25.0, 1.0, 25.0);
    floor_model_mat = Matrix4x4::new_translation(0.0, -1.0, 0.0) * floor_model_mat;
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &floor_model_mat);
    gl::DrawArrays(gl::TRIANGLES, 12, 6);
    for matrix in &CUBES_MODEL_MATRICES {
        gfx.set_uniform_mat4x4("model_mat", shd_idx, matrix);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
    }

    // Render scene as normal
    gl::BindFramebuffer(FRAMEBUFFER, 0);
    gl::Viewport(0, 0, window.get_size().0, window.get_size().1);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    let tex_id = gfx.get_texture_attachment_gl_id("Depth Map");
    //let tex_id = gfx.get_texture_attachment_gl_id("Temp texture attachment");
    gl::BindTexture(TEXTURE_2D, tex_id);
    let shd_id = gfx.get_shader_program_gl_id("Depth Visualization shader");
    gl::UseProgram(shd_id);
    gl::DrawArrays(gl::TRIANGLES, 0, 6);
}

pub unsafe fn setup_shadow_mapping(gfx: &mut GlData, /*todo temp:*/ _window: &Window) {
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

    let light_projection_mat = Matrix4x4::new_orthographic_projection(20.0, 20.0, 7.5, 1.0);
    use crate::camera::Camera;
    let pos = Vector3::new(-2.0, 4.0, -1.0);
    let light_view_mat =
        Camera::calculate_look_at_matrix(pos, pos * -2, Vector3::new(0.0, 1.0, 0.0));
    let light_space_mat = light_projection_mat * light_view_mat;
    let shd_idx = gfx.get_shader_program_index("Depth/Shadow Map shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_mat4x4("light_space_mat", shd_idx, &light_space_mat);

    //let mut cube1_model_mat = Matrix4x4::new_scaling(0.5, 0.5, 0.5);
    let cube1_model_mat = /*cube1_model_mat * */Matrix4x4::new_translation(0.0, 1.5, 0.0);
    CUBES_MODEL_MATRICES.push(cube1_model_mat);
    //let mut cube2_model_mat = Matrix4x4::new_scaling(0.5, 0.5, 0.5);
    let cube2_model_mat = /*cube2_model_mat * */Matrix4x4::new_translation(2.0, 0.0, 1.0);
    CUBES_MODEL_MATRICES.push(cube2_model_mat);
    let mut cube3_model_mat = Matrix4x4::new_scaling(0.5, 0.5, 0.5);
    cube3_model_mat = Matrix4x4::new_rotation(60.0, !Vector3::new(1.0, 0.0, 1.0)) * cube3_model_mat;
    cube3_model_mat = Matrix4x4::new_translation(-1.0, 0.0, 2.0) * cube3_model_mat;
    CUBES_MODEL_MATRICES.push(cube3_model_mat);

    gl::BindVertexArray(gfx.vertex_array_objects[2]);

    //let shd_idx = gfx.get_shader_program_index("Depth Testing shader");
    let shd_idx = gfx.get_shader_program_index("Depth Visualization shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let quad_scaling = Matrix4x4::new_scaling(2.0, 2.0, 2.0);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &quad_scaling);
    let identity_mat = Matrix4x4::identity_matrix();
    gfx.set_uniform_mat4x4("view_mat", shd_idx, &identity_mat);
    gfx.set_uniform_mat4x4("projection_mat", shd_idx, &identity_mat);
    //gfx.set_uniform_1u("depth_visualization_mode", shd_idx, 1);

    gl::BindFramebuffer(FRAMEBUFFER, 0);
    /*super::super::framebuffers::create_framebuffer(
        gfx,
        window.get_size(),
        "Temp framebuffer",
        "Temp texture attachment",
        gl::NEAREST,
        gl::NEAREST, /* Rustfmt force vertical formatting */
    );*/
}
