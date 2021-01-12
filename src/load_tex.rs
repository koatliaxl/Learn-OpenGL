use crate::gl;
use crate::gl::types::{GLenum, GLint, GLuint};
use std::ffi::c_void;

pub unsafe fn generate_gl_texture(
    image_path: &str,
    wrap_s: GLenum,
    wrap_t: GLenum,
    min_filter: GLenum,
    mag_filter: GLenum,
) -> GLuint {
    let image = image::open(image_path)
        .expect("Failed to load image (texture)")
        .flipv()
        .to_rgba8();
    let (width, height) = image.dimensions();
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_2D, texture_id);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        image.as_ptr() as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
    texture_id
}

#[deprecated]
pub unsafe fn gen_gl_tex_and_set_shd_var(
    image_path: &str,
    shader_variable_name: &str,
    shader_variable_value: GLint,
    wrap_s: GLenum,
    wrap_t: GLenum,
    min_filter: GLenum,
    mag_filter: GLenum,
    shader_program_id: u32,
) -> GLuint {
    let image = image::open(image_path)
        .expect("Failed to load image (texture)")
        .flipv()
        .to_rgba8();
    let (width, height) = image.dimensions();
    let mut texture_id = 0;
    gl::GenTextures(1, &mut texture_id);
    gl::BindTexture(gl::TEXTURE_2D, texture_id);
    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        image.as_ptr() as *const c_void,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrap_s as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrap_t as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter as i32);
    let texture_variable_location = gl::GetUniformLocation(
        shader_program_id, /* Rustfmt force vertical formatting */
        shader_variable_name.as_ptr() as *const i8,
    );
    println!(
        "\"{1:}\" variable location: {}",
        texture_variable_location, shader_variable_name
    );
    gl::UseProgram(shader_program_id);
    gl::Uniform1i(texture_variable_location, shader_variable_value);
    texture_id
}
