use crate::draw::LightProjectionMatrix;
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
