use crate::gl;
use crate::gl::types::GLuint;
use crate::load_tex::generate_gl_texture;

#[allow(deprecated)]
pub fn init_textures(shader_programs: &Vec<GLuint>) -> Vec<GLuint> {
    println!();
    use crate::load_tex::gen_gl_tex_and_set_shd_var;
    unsafe {
        let wall_texture_id = gen_gl_tex_and_set_shd_var(
            "assets/wall.jpg",
            "Wall_texture\0",
            0,
            gl::REPEAT,
            gl::REPEAT,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::NEAREST,
            shader_programs[1],
        );
        let awf_texture_id = gen_gl_tex_and_set_shd_var(
            "assets/awesomeface.png",
            "AwF_texture\0",
            1,
            gl::MIRRORED_REPEAT,
            gl::REPEAT,
            gl::LINEAR,
            gl::LINEAR,
            shader_programs[1],
        );
        let con_texture_id = gen_gl_tex_and_set_shd_var(
            "assets/container.jpg",
            "Con_texture\0",
            2,
            gl::CLAMP_TO_EDGE,
            gl::CLAMP_TO_BORDER,
            gl::LINEAR,
            gl::LINEAR,
            shader_programs[1],
        );
        let con2_texture_id = gen_gl_tex_and_set_shd_var(
            "assets/container2.png",
            "material.texture\0",
            3,
            gl::CLAMP_TO_BORDER,
            gl::CLAMP_TO_BORDER,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
            shader_programs[2],
        );
        let con2_specular_map_id = gen_gl_tex_and_set_shd_var(
            "assets/container2_specular.png",
            "material.specular_map\0",
            4,
            gl::CLAMP_TO_BORDER,
            gl::CLAMP_TO_BORDER,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
            shader_programs[2],
        );
        let emission_map_id = gen_gl_tex_and_set_shd_var(
            "assets/matrix.jpg",
            "material.emission_map\0",
            5,
            gl::CLAMP_TO_BORDER,
            gl::CLAMP_TO_BORDER,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
            shader_programs[2],
        );
        let metal_texture_id = generate_gl_texture(
            "assets/metal.png",
            gl::REPEAT,
            gl::REPEAT,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
        );
        let marble_texture_id = generate_gl_texture(
            "assets/marble.jpg",
            gl::CLAMP_TO_BORDER,
            gl::CLAMP_TO_BORDER,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
        );
        /*let grass_texture_id = generate_gl_texture(
            "assets/grass.png",
            gl::CLAMP_TO_EDGE,
            gl::CLAMP_TO_EDGE,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
        );*/
        let window_texture_id = generate_gl_texture(
            "assets/blending_transparent_window.png",
            gl::CLAMP_TO_EDGE,
            gl::CLAMP_TO_EDGE,
            gl::LINEAR_MIPMAP_LINEAR,
            gl::LINEAR,
        );
        /*let wall_texture_id_2 = gen_texture(
            "assets/wall.jpg",
            "Wall_texture\0",
            0,
            gl::REPEAT,
            gl::REPEAT,
            gl::LINEAR_MIPMAP_NEAREST,
            gl::NEAREST,
            shader_programs[1],
        );
        let wall_texture_id_3 = gen_texture(
            "assets/wall.jpg",
            "Wall_texture\0",
            0,
            gl::REPEAT,
            gl::REPEAT,
            gl::NEAREST_MIPMAP_LINEAR,
            gl::NEAREST,
            shader_programs[1],
        );
        let wall_texture_id_4 = gen_texture(
            "assets/wall.jpg",
            "Wall_texture\0",
            0,
            gl::REPEAT,
            gl::REPEAT,
            gl::LINEAR,
            gl::NEAREST,
            shader_programs[1],
        );*/
        vec![
            wall_texture_id,
            awf_texture_id,
            con_texture_id,
            con2_texture_id,
            con2_specular_map_id,
            emission_map_id,
            metal_texture_id,
            marble_texture_id,
            //grass_texture_id,
            window_texture_id,
        ]
    }
}
