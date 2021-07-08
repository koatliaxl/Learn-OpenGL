#[allow(unused_imports)]
use crate::gl::{CLAMP_TO_BORDER, CLAMP_TO_EDGE, MIRRORED_REPEAT, REPEAT};
#[allow(unused_imports)]
use crate::gl::{
    LINEAR, LINEAR_MIPMAP_LINEAR, LINEAR_MIPMAP_NEAREST, NEAREST, NEAREST_MIPMAP_LINEAR,
    NEAREST_MIPMAP_NEAREST,
};
use crate::load_tex::load_texture;
use crate::state_and_cfg::GlData;

#[allow(deprecated)]
pub fn init_textures(gl_data: &mut GlData) {
    println!();
    use crate::load_tex::load_tex_and_set_shd_var;
    unsafe {
        let wall_texture_id = load_tex_and_set_shd_var(
            "assets/wall.jpg",
            "Wall_texture\0",
            0,
            REPEAT,
            REPEAT,
            LINEAR_MIPMAP_LINEAR,
            NEAREST,
            gl_data.shader_programs[1],
        );
        let awf_texture_id = load_tex_and_set_shd_var(
            "assets/awesomeface.png",
            "AwF_texture\0",
            1,
            MIRRORED_REPEAT,
            REPEAT,
            LINEAR,
            LINEAR,
            gl_data.shader_programs[1],
        );
        let con_texture_id = load_tex_and_set_shd_var(
            "assets/container.jpg",
            "Con_texture\0",
            2,
            CLAMP_TO_EDGE,
            CLAMP_TO_BORDER,
            LINEAR,
            LINEAR,
            gl_data.shader_programs[1],
        );
        let con2_texture_id = load_tex_and_set_shd_var(
            "assets/container2.png",
            "material.texture\0",
            3,
            CLAMP_TO_BORDER,
            CLAMP_TO_BORDER,
            LINEAR_MIPMAP_LINEAR,
            LINEAR,
            gl_data.shader_programs[2],
        );
        let con2_specular_map_id = load_tex_and_set_shd_var(
            "assets/container2_specular.png",
            "material.specular_map\0",
            4,
            CLAMP_TO_BORDER,
            CLAMP_TO_BORDER,
            LINEAR_MIPMAP_LINEAR,
            LINEAR,
            gl_data.shader_programs[2],
        );
        let emission_map_id = load_tex_and_set_shd_var(
            "assets/matrix.jpg",
            "material.emission_map\0",
            5,
            CLAMP_TO_BORDER,
            CLAMP_TO_BORDER,
            LINEAR_MIPMAP_LINEAR,
            LINEAR,
            gl_data.shader_programs[2],
        );
        gl_data.add_texture(wall_texture_id, "Wall texture");
        gl_data.add_texture(awf_texture_id, "AwF texture");
        gl_data.add_texture(con_texture_id, "Container texture");
        gl_data.add_texture(con2_texture_id, "Container 2 texture");
        gl_data.add_texture(con2_specular_map_id, "Container 2 specular map");
        gl_data.add_texture(emission_map_id, "Container 2 emission map");
        let textures_to_load = [
            (
                "assets/metal.png",
                "Metal texture",
                REPEAT,
                REPEAT,
                LINEAR_MIPMAP_LINEAR,
                LINEAR,
            ),
            (
                "assets/marble.jpg",
                "Marble texture",
                CLAMP_TO_BORDER,
                CLAMP_TO_BORDER,
                LINEAR_MIPMAP_LINEAR,
                LINEAR,
            ),
            (
                "assets/blending_transparent_window.png",
                "Window texture",
                CLAMP_TO_EDGE,
                CLAMP_TO_EDGE,
                LINEAR_MIPMAP_LINEAR,
                LINEAR,
            ),
            // Safe to change ordering after this
            (
                "assets/wood.png",
                "Wood Flooring",
                REPEAT,
                REPEAT,
                LINEAR_MIPMAP_LINEAR,
                LINEAR,
            ), /*(
                   "assets/grass.png",
                   CLAMP_TO_EDGE,
                   CLAMP_TO_EDGE,
                   LINEAR_MIPMAP_LINEAR,
                   LINEAR,
               ),*/
            (
                "assets/wood.png",
                "Wood Flooring (clamp to edge)",
                CLAMP_TO_EDGE,
                CLAMP_TO_EDGE,
                LINEAR_MIPMAP_LINEAR,
                LINEAR,
            ),
        ];
        for (path, name, wrap_s, wrap_t, min_filter, mag_filter) in &textures_to_load {
            let tex_gl_id = load_texture(path, *wrap_s, *wrap_t, *min_filter, *mag_filter);
            gl_data.add_texture(tex_gl_id, name);
        }
    }
}
