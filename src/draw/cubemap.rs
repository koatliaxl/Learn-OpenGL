use crate::camera::Camera;
use crate::gl;
use crate::gl::types::GLenum;
use crate::state_and_cfg::GlData;
use matrix::Matrix4x4;
use opengl_learn::Model;
use std::ffi::c_void;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum EnvironmentMappingMode {
    //DrawCubemap,
    Reflection,
    Refraction,
}

pub unsafe fn draw_cubemap_scene(
    gfx: &GlData,
    //view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    camera: &Camera,
    model: &Model,
    mode: EnvironmentMappingMode,
) {
    let shd_idx = gfx.get_shader_program_index("Environment Mapping shader");
    gl::UseProgram(gfx.shader_programs[shd_idx]);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &Matrix4x4::identity_matrix());
    gfx.set_uniform_mat4x4("view_mat", shd_idx, &camera.look_at_matrix);
    gfx.set_uniform_mat4x4("projection_mat", shd_idx, projection_matrix);
    gfx.set_uniform_1i("mode", shd_idx, mode.int_code());
    gfx.set_uniform_vec3f("camera_position", shd_idx, camera.position);
    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, gfx.textures[9]);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);

    let model_mat = Matrix4x4::new_translation(2.0, 1.0, -2.0);
    gfx.set_uniform_mat4x4("model_mat", shd_idx, &model_mat);
    model.draw();

    //gl::DepthMask(gl::FALSE);
    gl::DepthFunc(gl::LEQUAL);
    let cm_shd = gfx.get_shader_program_index("Cubemap shader");
    gl::UseProgram(gfx.shader_programs[cm_shd]);
    let mut no_translate_view = camera.look_at_matrix.clone();
    // remove the translation section from matrix
    for i in 0..3 {
        no_translate_view.set(i, 3, 0.0);
    }
    gfx.set_uniform_mat4x4("view_mat", cm_shd, &no_translate_view);
    gfx.set_uniform_mat4x4("projection_mat", cm_shd, projection_matrix);
    gl::BindVertexArray(gfx.vertex_array_objects[3]);
    gl::ActiveTexture(gl::TEXTURE0);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, gfx.textures[9]);
    gl::DrawElements(
        gl::TRIANGLES, /* Rustfmt force vertical formatting */
        36,
        gl::UNSIGNED_INT,
        0 as *const c_void,
    );
    //gl::DepthMask(gl::TRUE);
    gl::DepthFunc(gl::LESS);
}

pub unsafe fn setup_cubemap_scene(gfx: &mut GlData, model: &mut Model) {
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_CUBE_MAP, texture_id);
    #[allow(unused_imports)]
    use gl::{
        TEXTURE_CUBE_MAP_NEGATIVE_X, /* Left */
        TEXTURE_CUBE_MAP_NEGATIVE_Y, /* Bottom */
        TEXTURE_CUBE_MAP_NEGATIVE_Z, /* Front */
        TEXTURE_CUBE_MAP_POSITIVE_X, /* Right */
        TEXTURE_CUBE_MAP_POSITIVE_Y, /* Top */
        TEXTURE_CUBE_MAP_POSITIVE_Z, /* Back */
    };
    let facets = [
        "assets/skybox/right.jpg",
        "assets/skybox/left.jpg",
        "assets/skybox/top.jpg",
        "assets/skybox/bottom.jpg",
        "assets/skybox/front.jpg",
        "assets/skybox/back.jpg",
    ];
    for i in 0..6 {
        load_facet(facets[i], TEXTURE_CUBE_MAP_POSITIVE_X + i as u32);
    }
    use gl::{CLAMP_TO_EDGE, LINEAR, TEXTURE_CUBE_MAP};
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_S, CLAMP_TO_EDGE as i32);
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_T, CLAMP_TO_EDGE as i32);
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_WRAP_R, CLAMP_TO_EDGE as i32);
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_MIN_FILTER, LINEAR as i32);
    gl::TexParameteri(TEXTURE_CUBE_MAP, gl::TEXTURE_MAG_FILTER, LINEAR as i32);
    gfx.textures.push(texture_id);

    model.load_model("assets/backpack/", "backpack.obj");
    model.load_textures_to_gl(
        gl::CLAMP_TO_BORDER,
        gl::CLAMP_TO_BORDER,
        gl::LINEAR_MIPMAP_LINEAR,
        gl::LINEAR,
    );
    model.setup_draw();
}

unsafe fn load_facet(image_path: &str, target: GLenum) {
    let image = image::open(image_path)
        .expect("Failed to load image (cubemap)")
        //.flipv()
        .to_rgb8();
    let (width, height) = image.dimensions();
    gl::TexImage2D(
        target,
        0,
        gl::RGB as i32,
        width as i32,
        height as i32,
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        image.as_ptr() as *const c_void,
    );
}

impl EnvironmentMappingMode {
    fn int_code(&self) -> i32 {
        match self {
            //EnvironmentMappingMode::DrawCubemap => 0,
            EnvironmentMappingMode::Reflection => 1,
            EnvironmentMappingMode::Refraction => 2,
        }
    }
}
