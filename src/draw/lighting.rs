use crate::gl;
use crate::state_and_cfg::{GlData, State};
use matrix::{Matrix4x4, Vector3, Vector4};
//use std::ffi::c_void;

static LIGHTING_SCENE_ATMOSPHERE: LightingSceneAtmosphere = DEFAULT;

pub unsafe fn draw_lighting_scene(
    gfx: &GlData,
    view_matrix: &Matrix4x4<f32>,
    projection_matrix: &Matrix4x4<f32>,
    time: f32,
    #[allow(unused_variables)] state: &State,
) {
    let radial_pos = (time % 360.0) * 0.02;
    let (rp_sin, rp_cos) = (radial_pos.sin(), radial_pos.cos());
    let orbit_radius = 2.0;
    let rotation = Matrix4x4::new_y_rotation(radial_pos * 100.0);
    let orbit_inclination_coefficient = 1.0;
    let lsx = rp_sin * orbit_radius;
    let lsy = rp_sin * orbit_inclination_coefficient;
    let lsz = rp_cos * orbit_radius;
    let ls_view_pos = view_matrix * Vector4::new(lsx, lsy, lsz, 1.0);
    let model_mat = rotation;
    gl::UseProgram(gfx.shader_programs[2]);
    gfx.set_uniform_mat4x4("model_mat", 2, &model_mat);
    gfx.set_uniform_mat4x4("view_mat", 2, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 2, projection_matrix);
    gfx.set_uniform_vec3f("light.position", 2, ls_view_pos.into());
    let mut world_up_in_view = Vector4::new_xyz(0.0, 1.0, 0.0);
    world_up_in_view = view_matrix * world_up_in_view;
    gfx.set_uniform_vec3f("world_up", 2, world_up_in_view.into());
    for i in 0..LIGHT_CUBES_NUM {
        let (x, y, z) = LIGHT_CUBES_POSITIONS[i];
        let pos = Vector4::new(x, y, z, 1.0);
        let view_pos = view_matrix * pos;
        let name = format!("point_lights[{}].position", i);
        gfx.set_uniform_vec3f(&name, 2, view_pos.into());
    }

    gl::BindVertexArray(gfx.vertex_array_objects[2]);
    gl::ActiveTexture(gl::TEXTURE3);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[3]);
    gl::ActiveTexture(gl::TEXTURE4);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[4]);
    gl::ActiveTexture(gl::TEXTURE5);
    gl::BindTexture(gl::TEXTURE_2D, gfx.textures[5]);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
    let cube_positions = [
        (2.0, 5.0, -15.0),
        (-1.5, -2.2, -2.5),
        (-3.8, -2.0, -12.3),
        (2.4, -0.4, -3.5),
        (-1.7, 3.0, -7.5),
        (1.3, -2.0, -2.5),
        (1.5, 2.0, -2.5),
        (1.5, 0.2, -1.5),
        (-1.3, 1.0, -1.5),
    ];
    for (i, pos) in cube_positions.iter().enumerate() {
        let translation = Matrix4x4::new_translation(pos.0, pos.1, pos.2);
        let angle = 20.0 * i as f32;
        let rotation = Matrix4x4::new_rotation(
            angle, /* Rustfmt force vertical formatting */
            Vector3::new(1.0, 0.3, 0.5).normalize(),
        );
        let model_mat = translation * rotation;
        gl::UniformMatrix4fv(
            gfx.get_var_loc("model_mat", 2),
            1,
            gl::TRUE,
            model_mat.as_ptr(),
        );
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
    }

    let ls_scaling_mat = Matrix4x4::new_scaling(0.2, 0.2, 0.2);
    let ls_translation_mat = Matrix4x4::new_translation(lsx, lsy, lsz);
    let light_source_model_matrix = ls_translation_mat * &ls_scaling_mat;
    gl::UseProgram(gfx.shader_programs[3]);
    gfx.set_uniform_mat4x4("model_mat", 3, &light_source_model_matrix);
    gfx.set_uniform_mat4x4("view_mat", 3, view_matrix);
    gfx.set_uniform_mat4x4("projection_mat", 3, projection_matrix);
    gfx.set_uniform_3f("color", 3, 1.0, 1.0, 1.0);
    gl::DrawArrays(gl::TRIANGLES, 0, 36);
    for i in 0..LIGHT_CUBES_NUM {
        let (x, y, z) = LIGHT_CUBES_POSITIONS[i];
        let translation = Matrix4x4::new_translation(x, y, z);
        let model_mat = translation * &ls_scaling_mat;
        gfx.set_uniform_mat4x4("model_mat", 3, &model_mat);
        gfx.set_uniform_vec3f(
            "color",
            3,
            Vector3::from_tuple(LIGHTING_SCENE_ATMOSPHERE.light_cubes_colors[i]),
        );
        gl::DrawArrays(gl::TRIANGLES, 0, 36);
    }
}

const LIGHT_CUBES_NUM: usize = 4;
const LIGHT_CUBES_POSITIONS: [(f32, f32, f32); LIGHT_CUBES_NUM] = [
    (0.7, 0.2, 2.0),
    (2.3, -3.3, -4.0),
    (-4.0, 2.0, -12.0),
    (0.0, 0.0, -3.0),
];

type Color = (f32, f32, f32);

#[derive(PartialEq, Copy, Clone)]
struct LightingSceneAtmosphere {
    clear_color: Color,
    light_cubes_colors: [Color; LIGHT_CUBES_NUM],
    ambient_to_specular_ratio: f32,
}

#[allow(dead_code)]
static DEFAULT: LightingSceneAtmosphere =
    LightingSceneAtmosphere::new((0.04, 0.04, 0.08), (1.0, 1.0, 1.0));
#[allow(dead_code)]
static DESERT: LightingSceneAtmosphere = LightingSceneAtmosphere {
    clear_color: (0.8, 0.6, 0.4),
    light_cubes_colors: [
        (1.0, 0.6, 0.0),
        (1.0, 0.0, 0.0),
        (1.0, 0.6, 0.0),
        (1.0, 0.0, 0.0),
    ],
    ambient_to_specular_ratio: 0.2,
};
#[allow(dead_code)]
static FACTORY: LightingSceneAtmosphere =
    LightingSceneAtmosphere::new((0.1, 0.1, 0.11), (0.2, 0.2, 0.5));
#[allow(dead_code)]
static HORROR: LightingSceneAtmosphere =
    LightingSceneAtmosphere::new_2((0.0, 0.0, 0.0), (0.4, 0.1, 0.1), 0.05);
#[allow(dead_code)]
static BIOCHEMICAL_LAB: LightingSceneAtmosphere =
    LightingSceneAtmosphere::new((0.95, 0.95, 0.95), (0.45, 0.75, 0.4));

pub unsafe fn init_lighting_scene(gfx: &GlData) {
    gl::UseProgram(gfx.shader_programs[2]);
    gfx.set_uniform_3f("material.specular", 2, 0.5, 0.5, 0.5);
    gfx.set_uniform_1f("material.shininess", 2, 64.0);
    gfx.set_uniform_3f("default_light.ambient", 2, 0.2, 0.2, 0.2);
    gfx.set_uniform_3f("default_light.diffuse", 2, 0.5, 0.5, 0.5);
    gfx.set_uniform_3f("default_light.specular", 2, 1.0, 1.0, 1.0);
    gfx.set_uniform_1f("default_attenuation.constant_term", 2, 1.0);
    gfx.set_uniform_1f("default_attenuation.linear_term", 2, 0.09);
    gfx.set_uniform_1f("default_attenuation.quadratic_term", 2, 0.032);
    gfx.set_uniform_1f("flash_light.inner_cutoff", 2, 12_f32.to_radians().cos());
    gfx.set_uniform_1f("flash_light.outer_cutoff", 2, 18_f32.to_radians().cos());
    gfx.set_uniform_3f("flash_light.point_light.light.specular", 2, 1.0, 1.0, 1.0);
    gfx.set_uniform_3f("flash_light.point_light.light.ambient", 2, 0.2, 0.2, 0.2);
    for i in 0..LIGHT_CUBES_NUM {
        let color = Vector3::from_tuple(LIGHTING_SCENE_ATMOSPHERE.light_cubes_colors[i]);
        let name = format!("point_lights[{}].light.specular", i);
        gfx.set_uniform_vec3f(&name, 2, color);
        let color = color * LIGHTING_SCENE_ATMOSPHERE.ambient_to_specular_ratio;
        let name = format!("point_lights[{}].light.ambient", i);
        gfx.set_uniform_vec3f(&name, 2, color);
    }
    let (cr, cg, cb) = LIGHTING_SCENE_ATMOSPHERE.clear_color;
    gl::ClearColor(cr, cg, cb, 1.0);
    if DEFAULT == LIGHTING_SCENE_ATMOSPHERE {
        gfx.set_uniform_3f("flash_light.point_light.light.specular", 2, 1.0, 0.2, 0.4);
    }
    if DESERT == LIGHTING_SCENE_ATMOSPHERE {
        gfx.set_uniform_3f("flash_light.point_light.light.specular", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_3f("flash_light.point_light.light.ambient", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_3f("default_light.ambient", 2, 0.5, 0.5, 0.3);
        gfx.set_uniform_3f("default_light.diffuse", 2, 1.0, 1.0, 0.8);
    }
    if FACTORY == LIGHTING_SCENE_ATMOSPHERE {
        gfx.set_uniform_3f("default_light.ambient", 2, 0.01, 0.01, 0.01);
        gfx.set_uniform_3f("default_light.diffuse", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_3f("default_light.specular", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_1f("flash_light.inner_cutoff", 2, 12_f32.to_radians().cos());
        gfx.set_uniform_1f("flash_light.outer_cutoff", 2, 15_f32.to_radians().cos());
        gfx.set_uniform_1f("default_attenuation.linear_term", 2, 0.05);
        gfx.set_uniform_1f("default_attenuation.quadratic_term", 2, 0.02);
    }
    if HORROR == LIGHTING_SCENE_ATMOSPHERE {
        gfx.set_uniform_1f("default_attenuation.linear_term", 2, 0.1);
        gfx.set_uniform_1f("default_attenuation.quadratic_term", 2, 0.05);
        gfx.set_uniform_3f("default_light.ambient", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_3f("default_light.diffuse", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_3f("default_light.specular", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_3f("flash_light.point_light.light.ambient", 2, 0.0, 0.0, 0.0);
    }
    if BIOCHEMICAL_LAB == LIGHTING_SCENE_ATMOSPHERE {
        gfx.set_uniform_3f("flash_light.point_light.light.specular", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_3f("flash_light.point_light.light.ambient", 2, 0.0, 0.0, 0.0);
        gfx.set_uniform_1f("default_attenuation.linear_term", 2, 0.2);
        gfx.set_uniform_1f("default_attenuation.quadratic_term", 2, 0.02);
        gfx.set_uniform_3f("default_light.ambient", 2, 0.3, 0.3, 0.3);
        gfx.set_uniform_3f("default_light.diffuse", 2, 0.8, 0.8, 0.8);
    }
}

impl LightingSceneAtmosphere {
    #[allow(dead_code)]
    const fn new(clear_color: Color, light_cubes_color: Color) -> LightingSceneAtmosphere {
        LightingSceneAtmosphere {
            clear_color,
            light_cubes_colors: [
                light_cubes_color,
                light_cubes_color,
                light_cubes_color,
                light_cubes_color,
            ],
            ambient_to_specular_ratio: 0.5,
        }
    }
    #[allow(dead_code)]
    const fn new_2(
        clear_color: Color,
        light_cubes_color: Color,
        ambient_to_specular_ratio: f32,
    ) -> LightingSceneAtmosphere {
        LightingSceneAtmosphere {
            clear_color,
            light_cubes_colors: [
                light_cubes_color,
                light_cubes_color,
                light_cubes_color,
                light_cubes_color,
            ],
            ambient_to_specular_ratio,
        }
    }
}
