use crate::state_and_cfg::State;
use crate::ShadowMappingSettings;
use glfw::Key;

pub fn change_shadow_mapping_settings(state: &mut State, key: Key) {
    let ShadowMappingSettings {
        min_shadow_bias,
        max_shadow_bias,
        cull_front_faces,
    } = &mut state.shadow_settings;
    match key {
        Key::N => {
            *min_shadow_bias -= 0.0001;
            println!("Min Shadow Bias: {}", min_shadow_bias);
            if *min_shadow_bias < 0.0 {
                *min_shadow_bias = 0.0;
            }
        }
        Key::M => {
            *min_shadow_bias += 0.0001;
            println!("Min Shadow Bias: {}", min_shadow_bias);
            if *min_shadow_bias > *max_shadow_bias {
                *min_shadow_bias = *max_shadow_bias;
            }
        }
        Key::Comma => {
            *max_shadow_bias -= 0.0005;
            println!("Max Shadow Bias: {}", max_shadow_bias);
            if *max_shadow_bias < *min_shadow_bias {
                *max_shadow_bias = *min_shadow_bias
            }
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
        _ => unreachable!(),
    }
}
