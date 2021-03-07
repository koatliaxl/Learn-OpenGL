use crate::gl;
use crate::gl::types::{GLint, GLuint};
use std::collections::HashMap;

pub fn get_variable_locations_2(
    shader_program_indexes: &HashMap<String, usize>,
    shader_programs: &Vec<GLuint>,
    variable_locations: &mut Vec<HashMap<String, GLint>>,
) {
    let variables = [
        ("UBO Use shader 1", vec!["model_mat"]),
        ("UBO Use shader 2", vec!["model_mat"]),
        ("UBO Use shader 3", vec!["model_mat"]),
        ("Geometry Shader Use 1", vec!["model_mat"]),
        ("Explode Effect shader", vec!["model_mat", "time"]),
        ("Draw Normals shader", vec!["model_mat", "rgba"]),
        ("UB Default shader", vec!["model_mat"]),
    ];
    let variables = variables
        .iter()
        .map(|(shd_name, var_names)| {
            let var_names = var_names
                .iter()
                .map(|var_name| var_name.to_string())
                .collect::<Vec<String>>();
            (shd_name.to_string(), var_names)
        })
        .collect::<Vec<(String, Vec<String>)>>();
    for _ in 0..variables.len() {
        variable_locations.push(HashMap::new());
    }
    for (shd_name, var_names) in variables {
        let shd_index = shader_program_indexes.get(&shd_name).unwrap();
        for mut var_name in var_names {
            var_name += "\0";
            let var_location = unsafe {
                gl::GetUniformLocation(
                    shader_programs[*shd_index],
                    var_name.as_ptr() as *const i8, /* Rustfmt force vertical formatting */
                )
            };
            var_name.remove(var_name.len() - 1);
            println!(
                "(\"{2:}\") \"{}\" variable location: {}",
                var_name, var_location, shd_name
            );
            variable_locations[*shd_index].insert(var_name, var_location);
        }
    }
}

#[deprecated]
pub fn get_variable_locations(shader_programs: &Vec<GLuint>) -> Vec<HashMap<String, GLint>> {
    let variables = [
        ("in_color", 1),
        ("offset", 0),
        ("Zoom", 1),
        ("Mix", 1),
        ("Transform", 1),
        ("Model_mat", 1),
        ("View_mat", 1),
        ("Projection_mat", 1),
        ("model_mat", 2),
        ("view_mat", 2),
        ("projection_mat", 2),
        ("model_mat", 3),
        ("view_mat", 3),
        ("projection_mat", 3),
        //("viewer_position", 2),
        ("material.shininess", 2),
        ("default_light.ambient", 2),
        ("default_light.diffuse", 2),
        ("default_light.specular", 2),
        ("light.position", 2),
        ("default_attenuation.constant_term", 2),
        ("default_attenuation.linear_term", 2),
        ("default_attenuation.quadratic_term", 2),
        ("flash_light.inner_cutoff", 2),
        ("flash_light.outer_cutoff", 2),
        ("flash_light.point_light.light.specular", 2),
        ("flash_light.point_light.light.ambient", 2),
        ("world_up", 2),
        ("color", 3),
        ("model_mat", 4),
        ("view_mat", 4),
        ("projection_mat", 4),
        ("viewer_position", 4),
        ("shininess", 4),
        ("diffuse_texture", 4),
        ("specular_map", 4),
        ("model_mat", 5),
        ("view_mat", 5),
        ("projection_mat", 5),
        ("depth_visualization_mode", 5),
        ("color", 6),
        ("model_mat", 6),
        ("view_mat", 6),
        ("projection_mat", 6),
        ("model_mat", 7),
        ("view_mat", 7),
        ("projection_mat", 7),
        ("model_mat", 8),
        ("view_mat", 8),
        ("projection_mat", 8),
        ("mode", 8),
        ("view_mat", 9),
        ("projection_mat", 9),
        ("model_mat", 10),
        ("view_mat", 10),
        ("projection_mat", 10),
        ("mode", 10),
        ("camera_position", 10),
    ];
    let mut variables = variables
        .iter()
        .map(|(name, shd_prg_idx)| (name.to_string(), *shd_prg_idx))
        .collect::<Vec<(String, usize)>>();
    for i in 0..4 {
        let pos_name = format!("point_lights[{}].position", i);
        variables.push((pos_name, 2));
        let color_name = format!("point_lights[{}].light.specular", i);
        variables.push((color_name, 2));
        let color_name = format!("point_lights[{}].light.ambient", i);
        variables.push((color_name, 2));
    }
    for i in 0..1 {
        let pos_name = format!("point_lights[{}].position", i);
        variables.push((pos_name, 4));
    }
    let mut result = Vec::new();
    for _ in 0..shader_programs.len() {
        result.push(HashMap::new())
    }
    println!();
    for (mut name, shader_program_index) in variables {
        name += "\0";
        let var_location = unsafe {
            gl::GetUniformLocation(
                shader_programs[shader_program_index],
                name.as_ptr() as *const i8,
            )
        };
        name.remove(name.len() - 1);
        println!("\"{}\" variable location: {}", name, var_location);
        result[shader_program_index].insert(name, var_location);
    }
    result
}
