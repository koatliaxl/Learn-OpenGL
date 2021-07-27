use crate::gl;
use crate::gl::types::{GLsizei, GLuint};
use crate::gl::{CLAMP_TO_EDGE, TEXTURE_CUBE_MAP};
use crate::state_and_cfg::GlData;
use glfw::Window;
use mat_vec::{Matrix4x4, Vector3};

static SHADOW_WIDTH: GLsizei = 1024;
static SHADOW_HEIGHT: GLsizei = 1024;
static NEAR_PROJ_PLANE: f32 = 1.0;
static FAR_PROJ_PLANE: f32 = 25.0;
//static mut LIGHT_SPACE_TRANSFORMS: Vec<Matrix4x4<f32>> = Vec::new();
static mut DEPTH_CUBEMAP_FBO: GLuint = 0;

pub unsafe fn draw_point_shadows(gfx: &GlData, window: &Window) {
    // Render to depth cubemap
    gl::Viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
    gl::BindFramebuffer(gl::FRAMEBUFFER, DEPTH_CUBEMAP_FBO);
    gl::Clear(gl::DEPTH_BUFFER_BIT);

    // Render scene as normal
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    gl::Viewport(0, 0, window.get_size().0, window.get_size().1);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
}

pub unsafe fn setup_point_shadows(gfx: &mut GlData) {
    let mut depth_cubemap = 0;
    gl::GenTextures(1, &mut depth_cubemap);
    gl::BindTexture(TEXTURE_CUBE_MAP, depth_cubemap);
    for i in 0..6 {
        gl::TexImage2D(
            gl::TEXTURE_CUBE_MAP_POSITIVE_X + i,
            0,
            gl::DEPTH_COMPONENT as i32,
            SHADOW_WIDTH,
            SHADOW_HEIGHT,
            0,
            gl::DEPTH_COMPONENT,
            gl::FLOAT,
            std::ptr::null(),
        );
    }
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, CLAMP_TO_EDGE as i32);
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
    gfx.add_texture_attachment(depth_cubemap, "Depth Cubemap");

    let mut fbo = 0;
    gl::GenFramebuffers(1, &mut fbo);
    gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
    gl::FramebufferTexture(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, depth_cubemap, 0);
    gl::DrawBuffer(gl::NONE);
    gl::ReadBuffer(gl::NONE);
    gfx.add_framebuffer(fbo, "Depth Cubemap FBO");
    DEPTH_CUBEMAP_FBO = fbo;

    let light_pos = Vector3::new(-2.0, 2.0, 0.0);

    let aspect_ratio = SHADOW_WIDTH as f32 / SHADOW_HEIGHT as f32;
    let light_space_proj =
        Matrix4x4::new_perspective_projection(90.0, aspect_ratio, FAR_PROJ_PLANE, NEAR_PROJ_PLANE);
    let light_space_view_1 = Matrix4x4::new_LookAt_matrix(
        light_pos,
        light_pos + Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
    );
    let light_space_view_2 = Matrix4x4::new_LookAt_matrix(
        light_pos,
        light_pos + Vector3::new(-1.0, 0.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
    );
    let light_space_view_3 = Matrix4x4::new_LookAt_matrix(
        light_pos,
        light_pos + Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
    );
    let light_space_view_4 = Matrix4x4::new_LookAt_matrix(
        light_pos,
        light_pos + Vector3::new(0.0, -1.0, 0.0),
        Vector3::new(0.0, 0.0, -1.0),
    );
    let light_space_view_5 = Matrix4x4::new_LookAt_matrix(
        light_pos,
        light_pos + Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, -1.0, 0.0),
    );
    let light_space_view_6 = Matrix4x4::new_LookAt_matrix(
        light_pos,
        light_pos + Vector3::new(0.0, 0.0, -1.0),
        Vector3::new(0.0, -1.0, 0.0),
    );
    let mut light_space_matrices = [Matrix4x4::zero_matrix(); 6];
    light_space_matrices[0] = &light_space_proj * light_space_view_1;
    light_space_matrices[1] = &light_space_proj * light_space_view_2;
    light_space_matrices[2] = &light_space_proj * light_space_view_3;
    light_space_matrices[3] = &light_space_proj * light_space_view_4;
    light_space_matrices[4] = &light_space_proj * light_space_view_5;
    light_space_matrices[5] = &light_space_proj * light_space_view_6;
    /*LIGHT_SPACE_TRANSFORMS.push(&light_space_proj * light_space_view_1);
    LIGHT_SPACE_TRANSFORMS.push(&light_space_proj * light_space_view_2);
    LIGHT_SPACE_TRANSFORMS.push(&light_space_proj * light_space_view_3);
    LIGHT_SPACE_TRANSFORMS.push(&light_space_proj * light_space_view_4);
    LIGHT_SPACE_TRANSFORMS.push(&light_space_proj * light_space_view_5);
    LIGHT_SPACE_TRANSFORMS.push(&light_space_proj * light_space_view_6);*/

    let shd_idx = gfx.get_shader_program_index("Point Shadows shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    //gfx.set_uniform_3f("Light_Pos", shd_idx, );
    gfx.set_uniform_1f("Far_Plane", shd_idx, FAR_PROJ_PLANE);
    for i in 0..6 {
        let var_name = format!("light_space_matrices[{}]", i);
        gfx.set_uniform_mat4x4(&var_name, shd_idx, &light_space_matrices[i]);
    }
}
