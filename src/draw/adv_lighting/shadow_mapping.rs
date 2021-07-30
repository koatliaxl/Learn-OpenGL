use super::super::draw_floor;
use crate::gl;
use crate::gl::types::{GLsizei, GLuint};
use crate::gl::{FRAMEBUFFER, TEXTURE_2D};
use crate::state_and_cfg::{GlData, State};
use glfw::Window;
use mat_vec::{Matrix4x4, Vector3};

static SHADOW_WIDTH: GLsizei = 1024;
static SHADOW_HEIGHT: GLsizei = 1024;
static mut VISUALIZE_DEPTH_MAP: bool = false;
static FLOOR_SCALE: f32 = 25.0;
static LIGHT_POSITION: (f32, f32, f32) = (-2.0, 4.0, -1.0);

static mut CUBES_MODEL_MATRICES: Vec<Matrix4x4<f32>> = Vec::new();
static mut DEPTH_VISUALIZATION_SHADER: GLuint = 0;
static mut LIGHT_SOURCE_SHADER: GLuint = 0;
static mut DEPTH_MAP_FRAMEBUFFER: GLuint = 0;

pub struct ShadowMappingSettings {
    pub min_shadow_bias: f32,
    pub max_shadow_bias: f32,
    pub cull_front_faces: bool, // (during the shadow map generation)
    pub projection_matrix: LightProjectionMatrix,
    pub projection_fov: f32, // in degrees
}
#[derive(Copy, Clone)]
pub enum LightProjectionMatrix {
    Orthographic,
    Perspective,
}
impl ShadowMappingSettings {
    pub const DEFAULT_MIN_SHADOW_BIAS_FOR_PERSPECTIVE: f32 = 0.0004;
    pub const DEFAULT_MAX_SHADOW_BIAS_FOR_PERSPECTIVE: f32 = 0.006;
    pub const DEFAULT_MIN_SHADOW_BIAS_FOR_ORTHOGRAPHIC: f32 = 0.0026;
    pub const DEFAULT_MAX_SHADOW_BIAS_FOR_ORTHOGRAPHIC: f32 = 0.037;
}

pub unsafe fn draw_shadow_mapping(gfx: &GlData, window: &Window, state: &State) {
    // Render to the depth map
    gl::Viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
    gl::BindFramebuffer(FRAMEBUFFER, DEPTH_MAP_FRAMEBUFFER);
    gl::Clear(gl::DEPTH_BUFFER_BIT);
    let shd_idx = gfx.get_shader_program_index("Depth/Shadow Map shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    let light_space_mat = calc_light_space_matrix(&state.shadow_settings);
    gfx.set_uniform_mat4x4("light_space_mat", shd_idx, &light_space_mat);
    draw_floor(gfx, shd_idx, FLOOR_SCALE);
    if state.shadow_settings.cull_front_faces {
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::FRONT);
    }
    for matrix in &CUBES_MODEL_MATRICES {
        gfx.set_uniform_mat4x4("model_mat", shd_idx, matrix);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
    }
    if state.shadow_settings.cull_front_faces {
        //gl::CullFace(gl::BACK);
        gl::Disable(gl::CULL_FACE);
    }

    // Render scene as normal
    gl::BindFramebuffer(FRAMEBUFFER, 0);
    gl::Viewport(0, 0, window.get_size().0, window.get_size().1);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
    if VISUALIZE_DEPTH_MAP {
        gl::UseProgram(DEPTH_VISUALIZATION_SHADER);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    } else {
        let shd_idx = gfx.get_shader_program_index("Shadow Mapping shader");
        gl::UseProgram(gfx.shader_programs[shd_idx]);
        gfx.set_uniform_vec3f("Viewer_Position", shd_idx, state.camera.position);
        gfx.set_uniform_1f(
            "min_shadow_bias",
            shd_idx,
            state.shadow_settings.min_shadow_bias,
        );
        gfx.set_uniform_1f(
            "max_shadow_bias",
            shd_idx,
            state.shadow_settings.max_shadow_bias,
        );
        gfx.set_uniform_mat4x4("light_space_matrix", shd_idx, &light_space_mat);
        draw_floor(gfx, shd_idx, FLOOR_SCALE);
        for matrix in &CUBES_MODEL_MATRICES {
            gfx.set_uniform_mat4x4("model_mat", shd_idx, matrix);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        gl::UseProgram(LIGHT_SOURCE_SHADER);
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
    }
}

unsafe fn calc_light_space_matrix(shadow_setting: &ShadowMappingSettings) -> Matrix4x4<f32> {
    use LightProjectionMatrix::*;
    let light_pos = Vector3::from_tuple(LIGHT_POSITION);
    let light_projection_mat = match shadow_setting.projection_matrix {
        Orthographic => Matrix4x4::new_orthographic_projection(20.0, 20.0, 7.5, 1.0),
        Perspective => {
            Matrix4x4::new_perspective_projection(shadow_setting.projection_fov, 1.0, 7.5, 1.0)
        }
    };
    let light_view_mat =
        Matrix4x4::new_LookAt_matrix(light_pos, -!light_pos, Vector3::new(0.0, 1.0, 0.0));
    light_projection_mat * light_view_mat
}

pub unsafe fn setup_shadow_mapping(gfx: &mut GlData, visualize_depth_map: bool) {
    let mut depth_map = 0;
    gl::GenFramebuffers(1, &mut depth_map);
    gl::BindFramebuffer(FRAMEBUFFER, depth_map);

    let mut depth_map_tex = 0;
    gl::GenTextures(1, &mut depth_map_tex);
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
    /*gl::TexParameteri(TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);*/
    gl::TexParameteri(TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
    gl::TexParameteri(TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
    let border_color = [1.0, 1.0, 1.0, 1.0];
    gl::TexParameterfv(TEXTURE_2D, gl::TEXTURE_BORDER_COLOR, border_color.as_ptr());

    gl::FramebufferTexture2D(
        FRAMEBUFFER,
        gl::DEPTH_ATTACHMENT,
        TEXTURE_2D,
        depth_map_tex,
        0,
    );
    gl::DrawBuffer(gl::NONE);
    gl::ReadBuffer(gl::NONE);
    DEPTH_MAP_FRAMEBUFFER = depth_map;
    gfx.add_framebuffer(depth_map, "Depth/Shadow Map");
    gfx.add_texture_attachment(depth_map_tex, "Depth Map");

    CUBES_MODEL_MATRICES.push(Matrix4x4::new_translation(0.0, 1.5, 0.0));
    CUBES_MODEL_MATRICES.push(Matrix4x4::new_translation(2.0, 0.0, 1.0));
    let mut cube3_model_mat = Matrix4x4::new_uniform_scaling(0.5);
    cube3_model_mat = Matrix4x4::new_rotation(60.0, !Vector3::new(1.0, 0.0, 1.0)) * cube3_model_mat;
    cube3_model_mat = Matrix4x4::new_translation(-1.0, 0.0, 2.0) * cube3_model_mat;
    CUBES_MODEL_MATRICES.push(cube3_model_mat);

    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    VISUALIZE_DEPTH_MAP = visualize_depth_map;
    gl::ActiveTexture(gl::TEXTURE1);
    gl::BindTexture(TEXTURE_2D, depth_map_tex);
    gl::ActiveTexture(gl::TEXTURE0);
    if visualize_depth_map {
        gl::BindTexture(TEXTURE_2D, depth_map_tex);
    } else {
        gl::BindTexture(TEXTURE_2D, gfx.get_texture_gl_id("Wood Flooring"));
    }
    //gl::Enable(gl::FRAMEBUFFER_SRGB);

    if visualize_depth_map {
        let shd_idx = gfx.get_shader_program_index("Depth Visualization shader");
        DEPTH_VISUALIZATION_SHADER = gfx.shader_programs[shd_idx];
        gl::UseProgram(DEPTH_VISUALIZATION_SHADER);
        let quad_scaling = Matrix4x4::new_uniform_scaling(2.0);
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &quad_scaling);
        let identity_mat = Matrix4x4::identity_matrix();
        gfx.set_uniform_mat4x4("view_mat", shd_idx, &identity_mat);
        gfx.set_uniform_mat4x4("projection_mat", shd_idx, &identity_mat);
    }

    let light_pos = Vector3::from_tuple(LIGHT_POSITION);

    let shd_idx = gfx.get_shader_program_index("Shadow Mapping shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_1i("Light_Sources_Num", shd_idx, 1);
    gfx.set_uniform_vec3f("Light_Sources[0].position", shd_idx, light_pos);
    let light_color = Vector3::new(1.0, 1.0, 1.0);
    gfx.set_uniform_vec3f("Light_Sources[0].color", shd_idx, light_color);
    gfx.set_uniform_1i("Shadow_Map", shd_idx, 1);

    if !visualize_depth_map {
        let shd_idx = gfx.get_shader_program_index("Single Color shader");
        LIGHT_SOURCE_SHADER = gfx.shader_programs[shd_idx];
        gl::UseProgram(LIGHT_SOURCE_SHADER);
        let scaling = Matrix4x4::new_uniform_scaling(0.1);
        let (lx, ly, lz) = light_pos.get_components();
        let translation = Matrix4x4::new_translation(lx, ly, lz);
        let model_mat = translation * scaling;
        gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
        gfx.set_uniform_vec3f("color", shd_idx, light_color);
    }
}
