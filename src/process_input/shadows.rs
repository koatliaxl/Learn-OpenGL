use crate::draw::{LightProjectionMatrix, OmnidirectionalShadowMappingSetting};
use crate::state_and_cfg::State;
use crate::ShadowMappingSettings;
use glfw::Key;
use LightProjectionMatrix::*;

pub fn change_shadow_mapping_settings(state: &mut State, key: Key) {
    let ShadowMappingSettings {
        min_shadow_bias,
        max_shadow_bias,
        cull_front_faces,
        projection_matrix,
        projection_fov,
    } = &mut state.shadow_settings;
    match key {
        Key::N => {
            *min_shadow_bias -= 0.0001;
            if *min_shadow_bias < 0.0 {
                *min_shadow_bias = 0.0;
            }
            println!("Min Shadow Bias: {}", min_shadow_bias);
        }
        Key::M => {
            *min_shadow_bias += 0.0001;
            if *min_shadow_bias > *max_shadow_bias {
                *min_shadow_bias = *max_shadow_bias;
            }
            println!("Min Shadow Bias: {}", min_shadow_bias);
        }
        Key::Comma => {
            *max_shadow_bias -= 0.0005;
            if *max_shadow_bias < *min_shadow_bias {
                *max_shadow_bias = *min_shadow_bias
            }
            println!("Max Shadow Bias: {}", max_shadow_bias);
        }
        Key::Period => {
            *max_shadow_bias += 0.0005;
            println!("Max Shadow Bias: {}", max_shadow_bias);
        }
        Key::L => {
            *cull_front_faces = !*cull_front_faces;
            match cull_front_faces {
                true => println!("Enabled front face culling during shadow map generation"),
                false => println!("Disabled front face culling during shadow map generation"),
            };
        }
        Key::P => {
            match projection_matrix {
                Orthographic => {
                    *projection_matrix = Perspective;
                    *min_shadow_bias =
                        ShadowMappingSettings::DEFAULT_MIN_SHADOW_BIAS_FOR_PERSPECTIVE;
                    *max_shadow_bias =
                        ShadowMappingSettings::DEFAULT_MAX_SHADOW_BIAS_FOR_PERSPECTIVE;
                    println!("Use perspective projection for shadow mapping");
                }
                Perspective => {
                    *projection_matrix = Orthographic;
                    *min_shadow_bias =
                        ShadowMappingSettings::DEFAULT_MIN_SHADOW_BIAS_FOR_ORTHOGRAPHIC;
                    *max_shadow_bias =
                        ShadowMappingSettings::DEFAULT_MAX_SHADOW_BIAS_FOR_ORTHOGRAPHIC;
                    println!("Use orthographic projection for shadow mapping");
                }
            }
            println!(
                "Min Shadow Bias: {}\nMax Shadow Bias: {}",
                min_shadow_bias, max_shadow_bias
            );
        }
        Key::LeftBracket => {
            *projection_fov -= 1.0;
            if *projection_fov < 1.0 {
                *projection_fov = 1.0
            }
            println!("Shadow mapping - projection FOV: {}", projection_fov);
        }
        Key::RightBracket => {
            *projection_fov += 1.0;
            if *projection_fov > 179.0 {
                *projection_fov = 179.0
            }
            println!("Shadow mapping - projection FOV: {}", projection_fov);
        }
        _ => unreachable!(),
    }
}

pub fn change_point_shadow_settings(state: &mut State, key: Key) {
    let OmnidirectionalShadowMappingSetting {
        visualize_depth_buffer,
        pcf,
        bias,
        pcf_disk_radius,
        disk_based_on_view_distance,
    } = &mut state.point_shadow_settings;

    match key {
        Key::V => {
            *visualize_depth_buffer = !*visualize_depth_buffer;
            match visualize_depth_buffer {
                true => println!("Enabled depth buffer visualization"),
                false => println!("Disabled depth buffer visualization"),
            }
        }
        Key::Comma => {
            *bias -= 0.002;
            if *bias < 0.0 {
                *bias = 0.0
            }
            println!("Shadow bias: {}", bias);
        }
        Key::Period => {
            *bias += 0.002;
            println!("Shadow bias: {}", bias);
        }
        Key::P => {
            *pcf = !*pcf;
            match pcf {
                true => println!("Enabled percentage-closer filtering"),
                false => println!("Disabled percentage-closer filtering"),
            }
        }
        Key::LeftBracket => {
            *pcf_disk_radius -= 0.002;
            if *pcf_disk_radius < 0.0 {
                *pcf_disk_radius = 0.0
            }
            println!("PCF sample disk radius: {}", pcf_disk_radius);
        }
        Key::RightBracket => {
            *pcf_disk_radius += 0.002;
            println!("PCF sample disk radius: {}", pcf_disk_radius);
        }
        Key::Apostrophe => {
            *disk_based_on_view_distance = !*disk_based_on_view_distance;
            match disk_based_on_view_distance {
                true => println!("PCF disk radius based on view distance"),
                false => println!("PCF disk radius is constant"),
            }
        }
        _ => unreachable!(),
    }
}
