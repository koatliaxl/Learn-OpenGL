use crate::gl;
use crate::gl::types::{GLsizei, GLuint};
use crate::gl::{CLAMP_TO_EDGE, TEXTURE_CUBE_MAP};
use crate::state_and_cfg::{GlData, State};
use glfw::Window;
use mat_vec::{Matrix4x4, Vector3};

static SHADOW_WIDTH: GLsizei = 1024;
static SHADOW_HEIGHT: GLsizei = 1024;
static NEAR_PROJ_PLANE: f32 = 1.0;
static FAR_PROJ_PLANE: f32 = 25.0;
static mut CUBES_MODEL_MATRICES: Vec<Matrix4x4<f32>> = Vec::new();
static mut DEPTH_CUBEMAP_FBO: GLuint = 0;
static mut LIGHT_SOURCE_SHADER: GLuint = 0;

pub struct OmnidirectionalShadowMappingSetting {
    pub visualize_depth_buffer: bool,
    pub pcf: bool,
    pub bias: f32,
    pub pcf_disk_radius: f32,
    pub disk_based_on_view_distance: bool, // PCF sample disk radius
}

pub unsafe fn draw_point_shadows(gfx: &GlData, window: &Window, state: &State) {
    // Render to depth cubemap
    gl::Viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
    gl::BindFramebuffer(gl::FRAMEBUFFER, DEPTH_CUBEMAP_FBO);
    gl::Clear(gl::DEPTH_BUFFER_BIT);
    let shd_idx = gfx.get_shader_program_index("Depth cubemap shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    render_scene(&gfx, shd_idx);

    // Render scene as normal
    gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    gl::Viewport(0, 0, window.get_size().0, window.get_size().1);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    let shd_idx = gfx.get_shader_program_index("Point Shadows shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    set_uniforms(&gfx, shd_idx, state);
    render_scene(&gfx, shd_idx);

    gl::UseProgram(LIGHT_SOURCE_SHADER);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
}

unsafe fn set_uniforms(gfx: &GlData, shd_idx: usize, state: &State) {
    gfx.set_uniform_vec3f("Viewer_Position", shd_idx, state.camera.position);
    gfx.set_uniform_1i(
        "visualize_cubemap_depth_buffer",
        shd_idx,
        match state.point_shadow_settings.visualize_depth_buffer {
            true => 1,
            false => 0,
        },
    );
    gfx.set_uniform_1i(
        "PCF",
        shd_idx,
        match state.point_shadow_settings.pcf {
            true => 1,
            false => 0,
        },
    );
    gfx.set_uniform_1f("Shadow_Bias", shd_idx, state.point_shadow_settings.bias);
    gfx.set_uniform_1f(
        "PCF_Disk_Radius",
        shd_idx,
        match state.point_shadow_settings.disk_based_on_view_distance {
            false => state.point_shadow_settings.pcf_disk_radius,
            true => -1.0,
        },
    );
}

unsafe fn render_scene(gfx: &GlData, shd_idx: usize) {
    // Room cube
    let model_mat = Matrix4x4::new_uniform_scaling(10.0);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gfx.set_uniform_1i("reverse_normals", shd_idx, 1);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
    gfx.set_uniform_1i("reverse_normals", shd_idx, 0);
    // Cubes
    for matrix in &CUBES_MODEL_MATRICES {
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &matrix);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
    }
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

    let light_pos = Vector3::new(0.0, 0.0, 0.0);

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
    let mut light_space_matrices = Vec::with_capacity(6);
    light_space_matrices.push(&light_space_proj * light_space_view_1);
    light_space_matrices.push(&light_space_proj * light_space_view_2);
    light_space_matrices.push(&light_space_proj * light_space_view_3);
    light_space_matrices.push(&light_space_proj * light_space_view_4);
    light_space_matrices.push(&light_space_proj * light_space_view_5);
    light_space_matrices.push(&light_space_proj * light_space_view_6);

    let shd_idx = gfx.get_shader_program_index("Depth cubemap shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_vec3f("Light_Pos", shd_idx, light_pos);
    gfx.set_uniform_1f("Far_Plane", shd_idx, FAR_PROJ_PLANE);
    for i in 0..light_space_matrices.len() {
        let var_name = format!("light_space_matrices[{}]", i);
        gfx.set_uniform_mat4x4(&var_name, shd_idx, &light_space_matrices[i]);
    }

    let shd_idx = gfx.get_shader_program_index("Point Shadows shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_vec3f("Light_Source.position", shd_idx, light_pos);
    gfx.set_uniform_vec3f("Light_Source.color", shd_idx, Vector3::new(1.0, 1.0, 1.0));
    gfx.set_uniform_1f("Far_Plane", shd_idx, FAR_PROJ_PLANE);
    gfx.set_uniform_1i("Depth_Cubemap", shd_idx, 1);

    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, depth_cubemap);
    let gl_id = gfx.get_texture_gl_id("Wood Flooring");
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_2D, gl_id);

    CUBES_MODEL_MATRICES.push(Matrix4x4::new_translation(4.0, -3.5, 0.0));
    CUBES_MODEL_MATRICES
        .push(Matrix4x4::new_translation(2.0, 3.0, 1.0) * Matrix4x4::new_uniform_scaling(1.5));
    CUBES_MODEL_MATRICES.push(Matrix4x4::new_translation(-3.0, -1.0, 0.0));
    CUBES_MODEL_MATRICES.push(Matrix4x4::new_translation(-1.5, 1.0, 1.5));
    let scaling = Matrix4x4::new_uniform_scaling(1.5);
    let rotation = Matrix4x4::new_rotation(60.0, !Vector3::new(1.0, 0.0, 1.0));
    let translation = Matrix4x4::new_translation(-1.5, 2.0, -3.0);
    CUBES_MODEL_MATRICES.push(translation * rotation * scaling);

    let shd_idx = gfx.get_shader_program_index("Single Color shader");
    LIGHT_SOURCE_SHADER = gfx.shader_programs[shd_idx];
    gl::UseProgram(LIGHT_SOURCE_SHADER);
    let scaling = Matrix4x4::new_uniform_scaling(0.1);
    let (lx, ly, lz) = light_pos.get_components();
    let translation = Matrix4x4::new_translation(lx, ly, lz);
    let model_mat = translation * scaling;
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    gfx.set_uniform_vec3f("color", shd_idx, Vector3::new(1.0, 1.0, 1.0));
}
